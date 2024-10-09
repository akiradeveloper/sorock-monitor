use anyhow::Result;
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

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::new().run(&mut terminal)?;
    terminal.clear()?;
    ratatui::restore();
    Ok(app_result)
}

fn random_list(n: usize) -> Vec<Node> {
    let mut nodes = vec![];
    for i in 0..n {
        nodes.push(Node {
            name: format!("Node {}", i + 1),
            progress: rand::random::<u16>() % 100,
        });
    }
    nodes
}

#[derive(Default)]
struct AppState {
    list_state: tui_widget_list::ListState,
}

struct App {}
impl App {
    fn new() -> Self {
        Self {}
    }
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
        let nodes = random_list(100);
        let nodes_list = NodeList { nodes };

        StatefulWidget::render(nodes_list, area, buf, &mut state.list_state);
    }
}
impl App {
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

/// ui

#[derive(Clone)]
struct Node {
    name: String,
    progress: u16,
}
impl Widget for Node {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let outer_block = Block::default().borders(Borders::ALL).title(self.name);

        let inner_area = outer_block.inner(area);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Fill(self.progress),
                    Constraint::Fill(100 - self.progress),
                ]
                .as_ref(),
            )
            .split(inner_area);

        Gauge::default()
            .gauge_style(
                Style::default()
                    .fg(Color::Green)
                    .bg(Color::Black)
                    .add_modifier(Modifier::ITALIC),
            )
            .label("done")
            .ratio(1.0)
            .render(chunks[0], buf);

        Gauge::default()
            .gauge_style(
                Style::default()
                    .fg(Color::Red)
                    .bg(Color::Black)
                    .add_modifier(Modifier::ITALIC),
            )
            .label("not done")
            .ratio(1.0)
            .render(chunks[1], buf);
        outer_block.render(area, buf);
    }
}
struct NodeList {
    nodes: Vec<Node>,
}
impl StatefulWidget for NodeList {
    type State = tui_widget_list::ListState;
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) where
        Self: Sized,
    {
        let n = self.nodes.len();
        let builder = tui_widget_list::ListBuilder::new(move |ctx| {
            let selected = ctx.is_selected;
            let idx = ctx.index;
            let mut node = self.nodes[idx].clone();
            node.name = if selected {
                format!("> {}", node.name)
            } else {
                node.name
            };
            (node, 3)
        });
        let view = tui_widget_list::ListView::new(builder, n);

        StatefulWidget::render(view, area, buf, state);
    }
}
