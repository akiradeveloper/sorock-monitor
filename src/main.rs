use anyhow::Result;
use spin::RwLock;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::os::unix::net::SocketAddr;
use std::sync::Arc;
use std::{io, vec};

use clap::Parser;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Gauge};
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    widgets::{Borders, StatefulWidget, Widget},
    DefaultTerminal,
};
use std::time::Duration;
use tonic::transport::{Channel, Endpoint, Server, Uri};

mod mock;
mod model;
mod ui;

mod proto {
    tonic::include_proto!("sorock_monitor");
}

#[derive(Parser)]
enum SubCommand {
    Connect { addr: Uri, shard_id: u32 },
    Test { number: u8 },
}

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let model = match args.subcommand {
        SubCommand::Connect { addr, shard_id } => model::Model::connect(addr, shard_id),
        SubCommand::Test { number: 0 } => model::Model::test(),
        SubCommand::Test { number: 1 } => {
            let url = mock::launch_mock_server();
            model::Model::connect(url, 0)
        }
        _ => unreachable!(),
    };

    let mut terminal = ratatui::init();
    let app_result = App::new(model).run(&mut terminal)?;
    terminal.clear()?;
    ratatui::restore();

    Ok(app_result)
}

struct App {
    model: model::Model,
}
impl App {
    pub fn new(model: model::Model) -> Self {
        Self { model }
    }

    fn run(self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut app_state = AppState::default();
        loop {
            terminal.draw(|frame| {
                frame.render_stateful_widget(&self, frame.area(), &mut app_state);
            })?;

            if !event::poll(Duration::from_millis(100))? {
                continue;
            }

            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Up | KeyCode::Char('k') => app_state.list_state.previous(),
                        KeyCode::Down | KeyCode::Char('j') => app_state.list_state.next(),
                        _ => {}
                    }
                }
            }
        }
    }
}

#[derive(Default)]
struct AppState {
    list_state: tui_widget_list::ListState,
}
impl StatefulWidget for &App {
    type State = AppState;
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) where
        Self: Sized,
    {
        let nodes = {
            let mut out = vec![];
            let reader = &self.model.nodes.read();

            let min_index = reader
                .nodes
                .values()
                .map(|node_state| node_state.log_state.head_index)
                .min()
                .unwrap_or(0);
            let max_index = reader
                .nodes
                .values()
                .map(|node_state| node_state.log_state.last_index)
                .max()
                .unwrap_or(0);

            for (uri, node_state) in &reader.nodes {
                let log_state = &node_state.log_state;
                out.push(ui::node_list::Node {
                    name: uri.to_string(),
                    head_index: log_state.head_index,
                    snapshot_index: log_state.snapshot_index,
                    app_index: log_state.app_index,
                    commit_index: log_state.commit_index,
                    last_index: log_state.last_index,
                    min_max: ui::node_list::IndexRange {
                        min_index,
                        max_index,
                    },
                });
            }
            out.sort_by_key(|node| node.commit_index);
            out.reverse();
            out
        };
        let nodes_list = ui::node_list::NodeList::new(nodes);
        StatefulWidget::render(nodes_list, area, buf, &mut state.list_state);
    }
}
