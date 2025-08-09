mod graph;
use graph::{Map, NodeShape};

mod tui;
use tui::{App, AppState, TabsState};

mod tui_draw;
use tui_draw::render;

use color_eyre::Result;

use crossterm::event::KeyCode;

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
    let mut app = App::new("xrpg: eXtensible RPG");
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
                app.on_key(key.code);
            }
            if app.app_state == AppState::Quit {
                // restore terminal
                disable_raw_mode()?;
                execute!(
                    terminal.backend_mut(),
                    LeaveAlternateScreen,
                    DisableMouseCapture
                )?;
                terminal.show_cursor()?;

                return Ok(());
            }
        }
    };

    if let Err(err) = app_result {
        println!("{err:?}");
    }

    Ok(())
}
