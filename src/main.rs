
use std::io;
use std::thread;
use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::Alignment;
use ratatui::widgets::Borders;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

use table::*;

#[derive(Debug, Default)]
pub struct App {
    table: Table,
    exit: bool,
}

impl App {
    pub fn new(table: Table) -> Self {
        Self { table, exit: false }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let lines = self.table.into_lines();

        let paragraph = Paragraph::new(lines)
            .alignment(Alignment::Center).block(Block::default().title("Game of Life").borders(Borders::ALL));

        let centered_area = Rect::new(
            area.x + (area.width.saturating_sub(100)) / 2,
            area.y + (area.height.saturating_sub(100)) / 2,
            100.min(area.width),
            100.min(area.height),
        );

        paragraph.render(centered_area, buf);
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut table = Table::new(10, 10);
    table.set_points(1,     1).unwrap()
        .set_points(1, 2).unwrap()
        .set_points(2, 2).unwrap();

    let mut terminal = ratatui::init();
    let app_result = App::new(table).run(&mut terminal)?;
    ratatui::restore();
    Ok(app_result)
}