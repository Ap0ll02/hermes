use crate::package::Package;
use ratatui::{prelude::*, widgets::*};

pub struct App {
    pub packages: Vec<Package>,
    pub selected: usize,
    pub search_query: String,
    pub mode: InputMode,
    pub show_help: bool,
    pub confirm_msg: Option<ConfirmAction>,
    pub status_msg: Option<String>,
}

pub enum ConfirmAction {
    Install(String),
    Remove(String),
    Update,
    Downgrade(String),
}

pub enum InputMode {
    Normal,
    Search,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            packages: Vec::new(),
            selected: 0,
            search_query: String::new(),
            mode: InputMode::Search,
            show_help: false,
            confirm_msg: None,
            status_msg: None,
        }
    }

    pub fn next(&mut self) {
        if !self.packages.is_empty() {
            self.selected = (self.selected + 1) % self.packages.len();
        }
    }

    pub fn previous(&mut self) {
        if !self.packages.is_empty() {
            if self.selected > 0 {
                self.selected -= 1;
            } else {
                self.selected = self.packages.len() - 1;
            }
        }
    }
}

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Search bar
            Constraint::Min(0),    // Package list
            Constraint::Length(7), // Details
            Constraint::Length(3), // Status Bar
        ])
        .split(frame.area());

    // Mode
    let mode_text = match app.mode {
        InputMode::Normal => "NORMAL",
        InputMode::Search => "SEARCH",
    };
    // Search bar
    let search_text = match app.mode {
        InputMode::Search => format!("  Search: {}█", app.search_query),
        InputMode::Normal => format!("  Search: {}", app.search_query),
    };

    let search = Paragraph::new(search_text)
        .style(Style::default().fg(Color::Cyan))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue))
                .title(Span::styled(
                    format!(" 📦 Hermes Package Manager: {mode_text}"),
                    Style::default().fg(Color::Yellow).bold(),
                )),
        );
    frame.render_widget(search, chunks[0]);

    // Package list
    let items: Vec<ListItem> = app
        .packages
        .iter()
        .enumerate()
        .map(|(i, pkg)| {
            let style = if i == app.selected {
                Style::default()
                    .bg(Color::Rgb(89, 180, 250))
                    .fg(Color::Black)
            } else {
                Style::default()
            };

            ListItem::new(pkg.display_line()).style(style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Packages")
            .border_style(Style::default().fg(Color::Blue)),
    );
    frame.render_widget(list, chunks[1]);

    // Details panel
    if let Some(pkg) = app.packages.get(app.selected) {
        let details_text = format!(
            "Name:        {}\nRepo:        {}\nVersion:     {}\nInstalled:   {}\nDescription: {}",
            pkg.name,
            pkg.repo,
            pkg.version,
            if pkg.installed { "Yes ✓" } else { "No" },
            pkg.description
        );

        let details = Paragraph::new(details_text).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Details")
                .border_style(Style::default().fg(Color::Blue)),
        );
        frame.render_widget(details, chunks[2]);
    }

    // Status Bar
    let status_text = app
        .status_msg.as_deref()
        .unwrap_or("Ready");

    let status = Paragraph::new(status_text)
        .style(Style::default().fg(Color::Green))
        .block(Block::default().borders(Borders::ALL).title("Status"));
    frame.render_widget(status, chunks[3]);

    if let Some(ref action) = app.confirm_msg {
        draw_confirm_overlay(frame, action);
    }

    if app.show_help {
        draw_help_overlay(frame);
    }
}

pub fn draw_help_overlay(frame: &mut Frame) {
    let area = centered_rect(60, 60, frame.area());
    let help_text = vec![
        "⚡ Hermes - Package Manager Help",
        "",
        "Navigation:",
        "  ↑/k         Move up",
        "  ↓/j         Move down",
        "  /           Enter search mode",
        "  Esc         Exit search mode",
        "",
        "Actions:",
        "  i           Install selected package",
        "  r           Remove selected package",
        "  Enter       View package details",
        "  d           Downgrade selected package",
        "  u           Update packages (pacman -Syu)",
        "",
        "Other:",
        "  ?           Toggle this help",
        "  q           Quit Hermes",
        "",
        "Press ? to close this help",
    ];

    let help = Paragraph::new(help_text.join("\n"))
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .title(" Help ")
                .style(Style::default().bg(Color::Rgb(30, 30, 46))),
        );

    frame.render_widget(Clear, area);
    frame.render_widget(help, area);
}

fn centered_rect(
    percent_x: u16,
    percent_y: u16,
    r: ratatui::layout::Rect,
) -> ratatui::layout::Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(r);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn draw_confirm_overlay(frame: &mut Frame, action: &ConfirmAction) {
    let area = centered_rect(30, 42, frame.area());
    let (title, message) = match action {
        ConfirmAction::Install(pkg) => (
            " Confirm Installation ",
            format!(
                "Install package '{}'?\n\nPress 'y' to confirm, 'n' to cancel",
                pkg
            ),
        ),
        ConfirmAction::Remove(pkg) => (
            " Confirm Removal ",
            format!(
                "Remove package '{}'?\n\nPress 'y' to confirm, 'n' to cancel",
                pkg
            ),
        ),
        ConfirmAction::Update => (
            " Confirm Update ",
            "Update and Upgrade packages?\n\nPress 'y' to confirm, 'n' to cancel".to_string(),
        ),
        ConfirmAction::Downgrade(pkg) => (
            " Confirm Downgrade Selection ",
            format!(
                "Downgrade package: {}\n\nPress 'y' to confirm, 'n' to cancel",
                pkg
            ),
        ),
    };

    let confirm = Paragraph::new(message)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::LightMagenta))
                .title(title)
                .style(Style::default().fg(Color::Blue)),
        );
    frame.render_widget(Clear, area);
    frame.render_widget(confirm, area);
}
