mod commands;
mod package;
mod ui;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, prelude::*};
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut ui::App) -> io::Result<()> {
    loop {
        let draw_res = terminal.draw(|f| ui::draw(f, app));
        match draw_res {
            Ok(_) => {}
            Err(e) => {
                println!("Error: {e}");
                panic!("Idk how to return properly");
            }
        }
        if let Event::Key(key) = event::read()? {
            // Handle confirmation dialog
            if let Some(ref action) = app.confirm_msg {
                match key.code {
                    KeyCode::Char('y') | KeyCode::Char('Y') => {
                        // Return to Alt Screen
                        disable_raw_mode()?;
                        {
                            let mut out = io::stdout();
                            execute!(out, LeaveAlternateScreen,)?;
                        }
                        match action {
                            ui::ConfirmAction::Install(_) => {
                                println!("\nInstalling: {}", app.packages[app.selected].name)
                            }
                            ui::ConfirmAction::Remove(_) => {
                                println!("\nRemoving: {}", app.packages[app.selected].name)
                            }
                            ui::ConfirmAction::Update => {
                                println!("\nUpdating Packages:")
                            }
                        }
                        // Execute the action
                        let result = match action {
                            ui::ConfirmAction::Install(pkg) => commands::install_package(pkg),
                            ui::ConfirmAction::Remove(pkg) => commands::remove_package(pkg),
                            ui::ConfirmAction::Update => commands::update_packages(),
                        };

                        println!("\nPress Enter To Continue...");
                        std::io::stdin().read_line(&mut String::new())?;

                        enable_raw_mode()?;
                        {
                            let mut out = io::stdout();
                            execute!(out, EnterAlternateScreen,)?;
                        }
                        terminal.clear();

                        app.status_msg = Some(match result {
                            Ok(msg) => msg,
                            Err(msg) => format!("Error: {}", msg),
                        });

                        app.confirm_msg = None;

                        // Refresh search to update installed status
                        if !app.search_query.is_empty() {
                            if let Ok(results) = package::search_packages(&app.search_query) {
                                app.packages = results;
                            }
                        }
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                        app.confirm_msg = None;
                        app.status_msg = Some("Action cancelled".to_string());
                    }
                    _ => {}
                }
                continue;
            }
            match app.mode {
                ui::InputMode::Search => {
                    match key.code {
                        KeyCode::Char(c) => {
                            app.search_query.push(c);
                            // Search as you type
                            if app.search_query.len() > 2 {
                                if let Ok(results) = package::search_packages(&app.search_query) {
                                    app.packages = results;
                                    app.selected = 0;
                                }
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
                        KeyCode::Enter => {
                            app.mode = ui::InputMode::Normal;
                            if app.search_query.len() < 3 {
                                if let Ok(results) = package::search_packages(&app.search_query) {
                                    app.packages = results;
                                    app.selected = 0;
                                }
                            }
                        }
                        KeyCode::Down => app.next(),
                        KeyCode::Up => app.previous(),
                        _ => {}
                    }
                }

                ui::InputMode::Normal => match key.code {
                    KeyCode::Char('q') => {
                        if app.show_help {
                            app.show_help = false;
                        } else {
                            return Ok(());
                        }
                    }
                    KeyCode::Char('/') => {
                        app.mode = ui::InputMode::Search;
                        app.search_query.clear();
                    }
                    KeyCode::Char('i') => {
                        if let Some(pkg) = app.packages.get(app.selected) {
                            if !pkg.installed {
                                app.confirm_msg =
                                    Some(ui::ConfirmAction::Install(pkg.name.clone()));
                            } else {
                                app.status_msg = Some(format!("{} is already installed", pkg.name));
                            }
                        }
                    }
                    KeyCode::Char('r') => {
                        if let Some(pkg) = app.packages.get(app.selected) {
                            if pkg.installed {
                                app.confirm_msg = Some(ui::ConfirmAction::Remove(pkg.name.clone()));
                            } else {
                                app.status_msg = Some(format!("{} is not installed", pkg.name));
                            }
                        }
                    }
                    KeyCode::Char('u') => {
                        app.confirm_msg = Some(ui::ConfirmAction::Update);
                    }
                    KeyCode::Char('h') | KeyCode::Char('?') => app.show_help = true,
                    KeyCode::Down | KeyCode::Char('j') => app.next(),
                    KeyCode::Up | KeyCode::Char('k') => app.previous(),
                    _ => {}
                },
            }
        }
    }
}
