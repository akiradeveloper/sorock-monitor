use super::*;

pub struct ProgressChart {
    data: BTreeMap<Instant, u64>,
    start: Instant,
    end: Instant,
}
impl ProgressChart {
    pub fn new(data: BTreeMap<Instant, u64>, start: Instant, end: Instant) -> Self {
        Self { data, start, end }
    }
    fn to_relative_time(&self, t: Instant) -> u64 {
        let duration = t - self.start;
        duration.as_millis() as u64
    }
}
impl Widget for ProgressChart {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let lo_v = *self.data.values().min().unwrap_or(&0);
        let hi_v = *self.data.values().max().unwrap_or(&0);

        let dataseq: Vec<(f64, f64)> = {
            let mut out = vec![];
            for (&t, &v) in &self.data {
                let x = self.to_relative_time(t) as f64;
                let y = v as f64;
                out.push((x, y));
            }
            out
        };
        let dataset = Dataset::default()
            .marker(Marker::Braille)
            .style(Style::default().fg(Color::Yellow))
            .graph_type(GraphType::Line)
            .data(&dataseq);

        let x_axis = {
            let lo = self.to_relative_time(self.start);
            let hi = self.to_relative_time(self.end);
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .title("Time")
                .bounds([lo as f64, hi as f64])
                .labels(["-60s", "0s"])
        };
        let y_axis = Axis::default()
            .style(Style::default().fg(Color::Gray))
            .title("Commit Index")
            .bounds([lo_v as f64, hi_v as f64])
            .labels([lo_v.to_string(), hi_v.to_string()]);
        Chart::new(vec![dataset])
            .block(
                Block::default()
                    .title("Commit Progress")
                    .borders(Borders::ALL),
            )
            .x_axis(x_axis)
            .y_axis(y_axis)
            .render(area, buf);
    }
}
