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
