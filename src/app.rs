use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::Terminal;
use ratatui::backend::Backend;

use crate::ai::AiClient;
use crate::filesystem::FileSystem;
use crate::ui::Ui;

pub struct App {
    ui: Ui,
    ai_client: AiClient,
    filesystem: FileSystem,
    should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            ui: Ui::new(),
            ai_client: AiClient::new(),
            filesystem: FileSystem::new(),
            should_quit: false,
        }
    }

    pub async fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        while !self.should_quit {
            terminal.draw(|f| self.ui.render(f))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => self.should_quit = true,
                        KeyCode::Char('c')
                            if key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                        {
                            self.should_quit = true
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }
}
