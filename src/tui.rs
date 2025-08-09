use crossterm::event::KeyCode;
use ratatui::widgets::ListState;

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

pub struct MenuEntries<'a> {
    pub titles: Vec<&'a str>,
    pub list_state: ListState,
    pub index: usize,
}

//TODO: duplicate code, turn into a trait?
//NOTE: why not ListState? because we want to keep the index and entries together.
impl<'a> MenuEntries<'a> {
    pub fn new(titles: Vec<&'a str>) -> Self {
        let list_state = ListState::default().with_selected(Some(0));
        Self { 
            titles,
            list_state,
            index: 0,
        }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
        self.list_state.select(Some(self.index));
    }
    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
            self.list_state.select(Some(self.index));
        } else {
            self.index = self.titles.len() - 1;
            self.list_state.select(Some(self.index));
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppState {
    MainMenu,
    MainWindow,
    Quit,
}

// app ui specific data, game data is SEPARATE. i repeat, the ENGINE'S DATA IS SEPARATE.
pub struct App<'a> {
    pub title: &'a str,
    pub app_state: AppState,

    // *** main window ***
    pub tabs: TabsState<'a>,
    // *** main window

    // *** main menu ***
    pub main_menu_list: MenuEntries<'a>,
    // *** main menu ***
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> Self {
        App {
            title,
            app_state: AppState::MainMenu,
            tabs: TabsState::new(vec!["Alpha", "Bravo", "Charlie", "Delta"]),
            main_menu_list: MenuEntries::new(vec!["Start", "Options", "Quit"]),
        }
    }

    fn main_menu_on_key(&mut self, c: KeyCode) {
        match c {
            KeyCode::Up => self.main_menu_list.previous(),
            KeyCode::Down => self.main_menu_list.next(),
            KeyCode::Enter => {
                match self.main_menu_list.titles[self.main_menu_list.index] {
                    "Start" => self.app_state = AppState::MainWindow,
                    "Quit" => self.app_state = AppState::Quit,
                    _ => {} // NOTE TODO: this is where we add menu items, handle exhaustively
                }
            }
            _ => {}
        }
    }

    fn main_window_on_key(&mut self, c: KeyCode) {
        match c {
            KeyCode::Esc => self.app_state = AppState::MainMenu,
            KeyCode::Tab => self.tabs.next(),
            KeyCode::BackTab => self.tabs.previous(),
            _ => {} // other keys dont do nothing
        }
    }

    pub fn on_key(&mut self, c: KeyCode) {
        // NOTE: must exhaustively handle all states, no buts ands ors ifs
        match self.app_state {
            AppState::MainMenu => self.main_menu_on_key(c),
            AppState::MainWindow => self.main_window_on_key(c),
            AppState::Quit => {} // handled in main.rs
        }
    }

    pub fn on_tick(&mut self) {
        // placeholder function, this will tick forward widgets that need ticking. check ratatui's
        // demo for more information. (src/app.rs @ App impl)
    }
}
