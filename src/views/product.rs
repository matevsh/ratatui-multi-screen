use crate::app::App;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::{Block, Borders};
use crate::views::{Command, Screen};

pub struct ProductView {}

impl ProductView {
    pub fn new() -> Self {
        Self {}
    }
}

impl Screen for ProductView {
    fn draw(&self, f: &mut Frame, app: &App) {
        let chunks = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(f.size());

        let block = Block::default()
            .title("Product")
            .borders(Borders::ALL);

        f.render_widget(block, chunks[0]);
    }

    fn handle_event(&mut self, _event: KeyEvent) -> Option<Command> {
        None
        // Handle events specific to Home screen
    }
}
