use ratatui::crossterm::event::KeyEvent;
use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::{Block, Borders};
use ratatui::Frame;

use crate::app::App;
use crate::views::{Command, Screen};

#[derive(Clone)]
pub struct ProductView {}

impl ProductView {
    pub fn new() -> Self {
        Self {}
    }
}

impl Screen for ProductView {
    fn draw(&self, f: &mut Frame, app: &App) {
        let chunks = Layout::default()
            .constraints([Constraint::Percentage(50)].as_ref())
            .split(f.size());

        let block = Block::default().title("Product").borders(Borders::ALL);

        f.render_widget(block, chunks[0]);
    }

    fn handle_event(&self, _event: KeyEvent, app: &mut App) -> Option<Command> {
        None
        // Handle events specific to Home screen
    }
}
