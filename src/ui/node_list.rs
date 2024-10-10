use super::*;

pub struct IndexRange {
    min_index: u64,
    max_index: u64,
}
impl IndexRange {
    pub fn calc_from(nodes: &[Node]) -> Self {
        let mut min_index = u64::MAX;
        let mut max_index = 0;
        for node in nodes {
            min_index = min_index.min(node.head_index);
            max_index = max_index.max(node.last_index);
        }
        Self {
            min_index,
            max_index,
        }
    }
}

struct LogStripe {
    min_to_head: u16,
    head_to_snap: u16,
    snap_to_app: u16,
    app_to_commit: u16,
    commit_to_last: u16,
    last_to_max: u16,
}
impl LogStripe {
    pub fn from(node: &Node) -> Self {}
}

pub struct Node {
    pub name: String,
    pub head_index: u64,
    pub snapshot_index: u64,
    pub app_index: u64,
    pub commit_index: u64,
    pub last_index: u64,
    pub min_max: IndexRange,
}

impl Widget for Node {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let stripe = LogStripe::from(&self);

        let outer_block = Block::default().borders(Borders::ALL).title(self.name);

        let inner_area = outer_block.inner(area);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Fill(stripe.min_to_head),
                    Constraint::Fill(stripe.head_to_snap),
                    Constraint::Fill(stripe.snap_to_app),
                    Constraint::Fill(stripe.app_to_commit),
                    Constraint::Fill(stripe.commit_to_last),
                ]
                .as_ref(),
            )
            .split(inner_area);

        Gauge::default()
            .gauge_style(
                Style::default()
                    .fg(Color::Gray)
                    .bg(Color::Black)
                    .add_modifier(Modifier::ITALIC),
            )
            .label("gc")
            .ratio(1.0)
            .render(chunks[1], buf);

        Gauge::default()
            .gauge_style(
                Style::default()
                    .fg(Color::Green)
                    .bg(Color::Black)
                    .add_modifier(Modifier::ITALIC),
            )
            .label("app")
            .ratio(1.0)
            .render(chunks[2], buf);

        Gauge::default()
            .gauge_style(
                Style::default()
                    .fg(Color::Yellow)
                    .bg(Color::Black)
                    .add_modifier(Modifier::ITALIC),
            )
            .label("commit")
            .ratio(1.0)
            .render(chunks[3], buf);

        Gauge::default()
            .gauge_style(
                Style::default()
                    .fg(Color::Red)
                    .bg(Color::Black)
                    .add_modifier(Modifier::ITALIC),
            )
            .label("uncommit")
            .ratio(1.0)
            .render(chunks[4], buf);

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
