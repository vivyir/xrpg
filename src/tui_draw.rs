use ratatui::layout::{Constraint, Layout, Rect, Alignment, Flex};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{self, Span};
use ratatui::widgets::canvas::{self, Canvas, Circle, MapResolution, Rectangle};
use ratatui::widgets::{
    Axis, BarChart, Block, Cell, Chart, Dataset, Gauge, LineGauge, List, ListState, ListItem, Paragraph, Row,
    Sparkline, Table, Tabs, Wrap,
};
use ratatui::widgets::block::BorderType;
use ratatui::{Frame, symbols};
use ratatui::prelude::Direction;

use crate::tui::{App, AppState, TabsState};

pub fn render(frame: &mut Frame, app: &mut App) {
    match app.app_state {
        AppState::MainMenu => render_main_menu(frame, app),
        AppState::MainWindow => render_main_window(frame, app),
        _ => {}
    }
}

fn render_main_menu(frame: &mut Frame, app: &mut App) {
    let centered_rect = {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(10),  // top margin
                Constraint::Percentage(80),  // main content area
                Constraint::Percentage(10), // bottom margin
            ])
            .split(frame.size());
        
        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(10),  // left margin
                Constraint::Percentage(80), // center content
                Constraint::Percentage(10), // right margin
            ])
            .split(layout[1]);  // split the middle vertical area
        
        inner_layout[1]
    };

    let list_entries = app
        .main_menu_list
        .titles
        .iter()
        .map(|t| text::Line::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect::<List>()
        .block(Block::bordered().title(app.title).border_type(BorderType::Rounded).title_alignment(Alignment::Center))
        .highlight_style(Style::default().fg(Color::Yellow));

    let mut list_state = ListState::default().with_selected(Some(app.main_menu_list.index));

    frame.render_stateful_widget(list_entries, centered_rect, &mut list_state);
}

fn render_main_window(frame: &mut Frame, app: &mut App) {
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
