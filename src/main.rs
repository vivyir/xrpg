mod graph;
use graph::{Map, NodeShape};

use color_eyre::Result;

// *** BEGIN UI LOGIC ***
use crossterm::event::KeyCode;

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub const fn new(titles: Vec<&'a str>) -> Self {
        Self { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }
    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> Self {
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["Alpha", "Bravo", "Charlie", "Delta"]),
        }
    }

    pub fn on_tab(&mut self) {
        self.tabs.next();
    }

    pub fn on_backtab(&mut self) {
        self.tabs.previous();
    }

    pub fn on_key(&mut self, c: KeyCode) {
        match c {
            KeyCode::Esc => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
        // placeholder function, this will tick forward widgets that need ticking. check ratatui's
        // demo for more information. (src/app.rs @ App impl)
    }
}
// *** END UI LOGIC ***

// *** START UI DRAWING ***
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
// *** END UI DRAWING ***

use std::io;
use std::time::{Duration, Instant};
use std::error::Error;

use crossterm::event::{self, DisableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut map = Map::new();
    
    let town = map.add_node("Home Town", NodeShape::Circle);
    let dungeon = map.add_node("Dark Dungeon", NodeShape::Asymmetric);
    let cave = map.add_node("Dragon's Cave", NodeShape::Rhombus);
    let forest = map.add_node("Enchanted Forest", NodeShape::Stadium);
    let castle = map.add_node("Royal Castle", NodeShape::Hexagon);
    
    map.add_bidirectional_path(town, forest, 15);
    map.add_bidirectional_path(town, castle, 25);
    map.add_path(forest, dungeon, 45);  // one-way path
    map.add_bidirectional_path(dungeon, cave, 30);
    map.add_bidirectional_path(castle, dungeon, 10);
    
    println!("=== Initial Map ===");
    println!("{}", map.to_mermaid());
    
    println!(
        "Path from Town to Forest is bidirectional: {}",
        map.is_bidirectional(town, forest)
    );
    println!(
        "Path from Forest to Dungeon is bidirectional: {}",
        map.is_bidirectional(forest, dungeon)
    );
    
    map.remove_directed_path(forest, dungeon);
    println!("\n=== After removing Forest->Dungeon path ===");
    println!("{}", map.to_mermaid());
    
    map.remove_node(dungeon);
    println!("\n=== After removing Dungeon node ===");
    println!("{}", map.to_mermaid());
    
    let village = map.add_node("River Village", NodeShape::Rounded);
    map.add_bidirectional_path(town, village, 20);
    println!("\n=== After adding River Village ===");
    println!("{}", map.to_mermaid());

    // terminal set up
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, DisableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // app run
    let mut app = App::new("xrpg: Extensible RPG");
    let app_result: Result<(), Box<dyn Error>> = {
        let terminal = &mut terminal;
        let tick_rate = Duration::from_millis(250);
        let mut last_tick = Instant::now();
        loop {
            terminal.draw(|frame| render(frame, &mut app))?;

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if !event::poll(timeout)? {
                app.on_tick();
                last_tick = Instant::now();
                continue;
            }

            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Tab => app.on_tab(),
                    KeyCode::BackTab => app.on_backtab(),
                    _else => app.on_key(_else),
                }
            }
            if app.should_quit {
                return Ok(());
            }
        }
    };

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = app_result {
        println!("{err:?}");
    }

    Ok(())
}
