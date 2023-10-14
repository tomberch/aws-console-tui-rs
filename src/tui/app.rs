use crate::config::config::AWSConfig;

use super::{
    component::root::Root,
    event::{Event, EventHandler},
    term::Term,
};
use anyhow::{Context, Result};
use aws_config::SdkConfig;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::Rect;

pub struct AppState {
    pub aws_config: AWSConfig,
    pub profiles: Vec<String>,
    pub regions: Vec<String>,
    pub services: Vec<String>,
    pub selected_profile: String,
    pub sdk_config: Option<SdkConfig>,
}

pub struct App<'a> {
    term: Term,
    events: EventHandler,
    root_component: Root<'a>,
    should_quit: bool,
}

impl<'a> App<'a> {
    pub fn new(root_component: Root<'a>) -> Result<Self> {
        Ok(Self {
            term: Term::start()?,
            events: EventHandler::new(),
            root_component,
            should_quit: false,
        })
    }

    pub fn run(mut self) -> Result<()> {
        while !self.should_quit {
            self.draw().context("Tried to draw the application")?;
            match self.events.next().context("Tried match event")? {
                Event::Tick => {}
                Event::Key(key_event) => self.handle_key_event(key_event)?,
                Event::Mouse(_) => {}
                Event::Resize(width, height) => {
                    self.term
                        .resize(Rect::new(0, 0, width, height))
                        .context("Tried to resize terminal window")?;
                }
            };
        }

        // Exit the user interface.
        Term::stop()?;
        Ok(())
    }

    fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn draw(&mut self) -> Result<()> {
        self.term
            .draw(|frame| self.root_component.render(frame, frame.size()))?;
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        if key.kind != KeyEventKind::Press {
            return Ok(());
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.quit();
            }

            KeyCode::Char('c') | KeyCode::Char('C') => self.quit(),
            _ => self.root_component.handle_key_event(key),
        };

        Ok(())
    }
}
