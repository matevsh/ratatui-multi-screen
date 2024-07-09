use std::any::Any;
use std::{error::Error, io};

use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use ratatui::crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::crossterm::{event, execute};
use ratatui::{backend::CrosstermBackend, Terminal};

use app::App;

mod app;
mod views;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut app = App::new();

    println!("Current view: {:?}", app.view.type_id());
    loop {
        terminal.draw(|f| app.view.draw(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                _ => app.clone().view.handle_event(key, &mut app),
            };
        }
    }
}
