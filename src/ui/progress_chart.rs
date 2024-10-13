use ratatui::widgets::Dataset;

use super::*;

pub struct ProgressChart {}
impl ProgressChart {
    pub fn new() -> Self {
        todo!()
    }
}
impl Widget for ProgressChart {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let dataseq: Vec<(f64, f64)> = { vec![] };
        let dataset = Dataset::default()
            .marker(Marker::Braille)
            .style(Style::default().fg(Color::Yellow))
            .data(&dataseq);
        Chart::new(vec![dataset])
            .block(
                Block::default()
                    .title("Commit Progress")
                    .borders(Borders::ALL),
            )
            .render(area, buf);
    }
}
