use super::{
    component::root::Root,
    event::{Event, EventHandler},
    term::Term,
};
use anyhow::{Context, Result};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::Rect;

#[derive(Debug)]
pub struct App {
    term: Term,
    events: EventHandler,
    should_quit: bool,
}

impl App {
    fn new() -> Result<Self> {
        Ok(Self {
            term: Term::start()?,
            events: EventHandler::new(),
            should_quit: false,
        })
    }

    pub fn run() -> Result<()> {
        let mut app = Self::new()?;

        while !app.should_quit {
            app.draw().context("Tried to draw the application")?;
            match app.events.next().context("Tried match event")? {
                Event::Tick => {}
                Event::Key(key_event) => app.handle_key_event(key_event)?,
                Event::Mouse(_) => {}
                Event::Resize(width, height) => {
                    app.term
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

    fn draw(&mut self) -> Result<()> {
        self.term
            .draw(|frame| frame.render_widget(Root::new(), frame.size()))
            .context("Tried to draw terminal")?;
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
            _ => {}
        };

        Ok(())
    }
}
