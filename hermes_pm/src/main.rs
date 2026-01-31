mod package;
mod ui;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, Terminal};
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    // Create app
    let mut app = ui::App::new();
    
    // Run app
    let res = run_app(&mut terminal, &mut app);
    
    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    
    if let Err(err) = res {
        println!("Error: {:?}", err);
    }
    
    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut ui::App,
) -> io::Result<()> {
    loop {
        let draw_res = terminal.draw(|f| ui::draw(f, app));
        match draw_res {
            Ok(_) => {},
            Err(e) => {
                println!("Error: {e}");
                panic!("Idk how to return properly");
            }
        } 
        if let Event::Key(key) = event::read()? {
            match app.mode {
                ui::InputMode::Search => {
                    match key.code {
                        KeyCode::Char(c) => {
                            app.search_query.push(c);
                            // Search as you type
                            if let Ok(results) = package::search_packages(&app.search_query) {
                                app.packages = results;
                                app.selected = 0;
                            }
                        }
                        KeyCode::Backspace => {
                            app.search_query.pop();
                            if let Ok(results) = package::search_packages(&app.search_query) {
                                app.packages = results;
                                app.selected = 0;
                            }
                        }
                        KeyCode::Esc => {
                            app.mode = ui::InputMode::Normal;
                        }
                        KeyCode::Down => app.next(),
                        KeyCode::Up => app.previous(),
                        _ => {}
                    }
                }
                ui::InputMode::Normal => {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('/') => {
                            app.mode = ui::InputMode::Search;
                            app.search_query.clear();
                        }
                        KeyCode::Down | KeyCode::Char('j') => app.next(),
                        KeyCode::Up | KeyCode::Char('k') => app.previous(),
                        _ => {}
                    }
                }
            }
        }
    }
}
