use crossterm::cursor::{Hide, MoveTo, MoveToRow, Show};
use crossterm::event::Event;
use crossterm::style::{Print, PrintStyledContent, Stylize};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear};
use crossterm::{event, execute, QueueableCommand};
use std::collections::VecDeque;
use std::io::{stdout, Write};
use std::iter::zip;
use termsize;

#[derive(Clone)]
pub struct UI {
    message: VecDeque<String>,
    info: String,
}

impl UI {
    pub fn new() -> UI {
        let _ = enable_raw_mode();
        UI {
            info: String::new(),
            message: VecDeque::new(),
        }
    }

    pub fn exit() -> std::io::Result<()> {
        disable_raw_mode()?;
        execute!(std::io::stdout(), Show)?;
        Ok(())
    }

    pub fn set_info(&mut self, msg: String) {
        self.info = msg.to_string();
    }

    pub fn push(&mut self, msg: String) {
        self.message.push_front(msg);
        if self.message.len() > 100 {
            self.message.pop_back();
        }
    }

    pub fn push_draw(&mut self, msg: String) {
        self.message.push_front(msg);
        if self.message.len() > 100 {
            self.message.pop_back();
        }
        let _ = self.draw();
    }

    pub fn draw(&self) -> std::io::Result<()> {
        let mut term = stdout();
        let size = match termsize::get() {
            Some(size) => size,
            None => panic!("Failed to get terminal size!"),
        };
        for row in (0..size.rows - 1).rev() {
            term.queue(MoveToRow(row))?
                .queue(Clear(crossterm::terminal::ClearType::CurrentLine))?;
        }
        term.queue(Hide)?
            .queue(MoveTo(0, size.rows - 2))?
            .queue(PrintStyledContent(self.info.clone().black().on_white()))?;

        for row in zip((0..=size.rows - 3).rev(), 0..=size.rows - 3) {
            term.queue(MoveTo(0, row.0))?;
            if usize::from(row.1) < self.message.len() {
                term.queue(Print(&self.message[row.1.into()]))?;
            }
        }
        term.flush()?;
        Ok(())
    }

    pub fn key(key: char) -> Result<bool, std::io::Error> {
        if event::poll(std::time::Duration::from_millis(0))? {
            match event::read()? {
                Event::Key(event) => match event.code {
                    event::KeyCode::Char(k) => {
                        if key == k {
                            Ok(true)
                        } else {
                            Ok(false)
                        }
                    }
                    _ => Ok(false),
                },
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    pub fn input() -> Result<String, std::io::Error> {
        let mut term = stdout();
        let size = match termsize::get() {
            Some(size) => size,
            None => panic!("Failed to get terminal size!"),
        };
        let mut input = String::new();
        let i = loop {
            let code = loop {
                match event::read()? {
                    Event::Key(event) => break event.code,
                    _ => continue,
                }
            };
            match code {
                event::KeyCode::Char(c) => input.push(c),
                event::KeyCode::Backspace => {
                    let _ = input.pop();
                }
                event::KeyCode::Enter => break input,
                _ => continue,
            }
            term.queue(MoveTo(0, size.rows))?
                .queue(Clear(crossterm::terminal::ClearType::CurrentLine))?
                .queue(Print(&input))?;
            term.flush()?;
        };
        term.queue(MoveTo(0, size.rows))?
            .queue(Clear(crossterm::terminal::ClearType::CurrentLine))?;
        Ok(i)
    }
}
