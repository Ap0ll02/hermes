use ratatui::{
    prelude::*,
    widgets::*,
};
use crate::package::Package;

pub struct App {
    pub packages: Vec<Package>,
    pub selected: usize,
    pub search_query: String,
    pub mode: InputMode,
}

pub enum InputMode {
    Normal,
    Search,
}

impl App {
    pub fn new() -> Self {
        Self {
            packages: Vec::new(),
            selected: 0,
            search_query: String::new(),
            mode: InputMode::Search,
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
            Constraint::Length(3),  // Search bar
            Constraint::Min(0),     // Package list
            Constraint::Length(7),  // Details
        ])
        .split(frame.area());
    
    // Search bar
    let search_text = match app.mode {
        InputMode::Search => format!("  Search: {}█", app.search_query),
        InputMode::Normal => format!("  Search: {}", app.search_query),
    };
    
    let search = Paragraph::new(search_text)
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue))
            .title(Span::styled(
                " ⚡ Hermes Package Manager ",
                Style::default().fg(Color::Yellow).bold()
            )));
    frame.render_widget(search, chunks[0]);
    
    // Package list
    let items: Vec<ListItem> = app.packages
        .iter()
        .enumerate()
        .map(|(i, pkg)| {
            let style = if i == app.selected {
                Style::default().bg(Color::Rgb(89, 180, 250)).fg(Color::Black)
            } else {
                Style::default()
            };
            
            ListItem::new(pkg.display_line()).style(style)
        })
        .collect();
    
    let list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Packages")
            .border_style(Style::default().fg(Color::Blue)));
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
        
        let details = Paragraph::new(details_text)
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Details")
                .border_style(Style::default().fg(Color::Blue)));
        frame.render_widget(details, chunks[2]);
    }
}
