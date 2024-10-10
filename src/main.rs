use anyhow::Result;
use http::Uri;
use spin::RwLock;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::{io, vec};

use ratatui::prelude::*;
use ratatui::widgets::{Block, Gauge};
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::{Borders, Paragraph, StatefulWidget, Widget},
    DefaultTerminal,
};
use std::time::Duration;

mod model;
mod stream;
mod ui;

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::test().run(&mut terminal)?;
    terminal.clear()?;
    ratatui::restore();
    Ok(app_result)
}

struct App {
    model: Arc<RwLock<model::Nodes>>,
}
impl App {
    pub fn test() -> Self {
        let mut nodes = model::Nodes::default();
        nodes.nodes.insert(
            Uri::from_static("http://unko:3000"),
            model::NodeState {
                log_state: model::LogState {
                    head_index: 100,
                    snapshot_index: 110,
                    app_index: 140,
                    commit_index: 160,
                    last_index: 165,
                },
            },
        );
        nodes.nodes.insert(
            Uri::from_static("http://kuso:3000"),
            model::NodeState {
                log_state: model::LogState {
                    head_index: 125,
                    snapshot_index: 130,
                    app_index: 140,
                    commit_index: 165,
                    last_index: 180,
                },
            },
        );
        Self {
            model: Arc::new(RwLock::new(nodes)),
        }
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
            let reader = &self.model.read();

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
            out
        };
        let nodes_list = ui::node_list::NodeList::new(nodes);
        StatefulWidget::render(nodes_list, area, buf, &mut state.list_state);
    }
}
