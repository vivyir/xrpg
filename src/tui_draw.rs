use ratatui::layout::{Constraint, Layout, Rect, Alignment};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{self, Span};
use ratatui::widgets::canvas::{self, Canvas, Circle, MapResolution, Rectangle};
use ratatui::widgets::{
    Axis, BarChart, Block, Cell, Chart, Dataset, Gauge, LineGauge, List, ListItem, Paragraph, Row,
    Sparkline, Table, Tabs, Wrap,
};
use ratatui::widgets::block::BorderType;
use ratatui::{Frame, symbols};

use crate::tui::{App, TabsState};

pub fn render(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(frame.area());
    let tabs = app
        .tabs
        .titles
        .iter()
        .map(|t| text::Line::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect::<Tabs>()
        .block(Block::bordered().title(app.title).border_type(BorderType::Rounded).title_alignment(Alignment::Center))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.index);
    frame.render_widget(tabs, chunks[0]);
    match app.tabs.index {
        _ => {}
    };
}
