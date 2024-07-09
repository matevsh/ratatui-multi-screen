use ratatui::crossterm::event::KeyEvent;
use ratatui::Frame;
use crate::app::{App, AppView};

pub mod menu;
pub mod product;

pub enum Command {
    SetScreen(AppView)
}

pub trait Screen {
    fn draw(&self, f: &mut Frame, app: &App);
    fn handle_event(&self, event: KeyEvent, app: &mut App) -> Option<Command>;
}