use anyhow::Result;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct Ui {
    input: String,
    messages: Vec<String>,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            messages: vec!["Welcome to fsbot-rs! Type your message and press Enter.".to_string()],
        }
    }

    pub fn render(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(f.size());

        // Messages area
        let messages: Vec<Spans> = self
            .messages
            .iter()
            .map(|m| Spans::from(vec![Span::raw(m)]))
            .collect();

        let messages = Paragraph::new(messages)
            .block(Block::default().borders(Borders::ALL).title("Chat"))
            .wrap(ratatui::widgets::Wrap { trim: true });

        f.render_widget(messages, chunks[0]);

        // Input area
        let input = Paragraph::new(self.input.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Input"));

        f.render_widget(input, chunks[1]);
        f.set_cursor(
            chunks[1].x + self.input.len() as u16 + 1,
            chunks[1].y + 1,
        );
    }
} 