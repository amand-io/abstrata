use std::env;
use std::fs;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

struct Editor {
    buffer: String,
    cursor: usize,
}

impl Editor {
    fn new() -> Editor {
        Editor {
            buffer: String::new(),
            cursor: 0,
        }
    }

    fn load_file(&mut self, filepath: &str) -> std::io::Result<()> {
        self.buffer = fs::read_to_string(filepath)?;
        self.cursor = self.buffer.len();
        Ok(())
    }

    fn insert(&mut self, c: char) {
        self.buffer.insert(self.cursor, c);
        self.cursor += 1;
    }

    fn delete(&mut self) {
        if self.cursor == self.buffer.len() {
            return;
        }
        self.buffer.remove(self.cursor);
    }

    fn backspace(&mut self) {
        if self.cursor == 0 {
            return;
        }
        self.buffer.remove(self.cursor - 1);
        self.cursor -= 1;
    }

    fn move_cursor_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    fn move_cursor_right(&mut self) {
        if self.cursor < self.buffer.len() {
            self.cursor += 1;
        }
    }

    fn save(&self, filepath: &str) -> std::io::Result<()> {
        fs::write(filepath, &self.buffer)?;
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let mut editor = Editor::new();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file path>", args[0]);
        std::process::exit(1);
    }

    let filepath = &args[1];
    editor.load_file(filepath)?;

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode()?;
    write!(stdout, "{}", termion::clear::All)?;

    for c in stdin.keys() {
        match c? {
            Key::Esc => break,
            Key::Char('\n') => editor.insert('\n'),
            Key::Char(c) => editor.insert(c),
            Key::Backspace => editor.backspace(),
            Key::Delete => editor.delete(),
            Key::Left => editor.move_cursor_left(),
            Key::Right => editor.move_cursor_right(),
            _ => (),
        }

        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )?;
        write!(stdout, "{}", editor.buffer)?;
        write!(
            stdout,
            "{}",
            termion::cursor::Goto(1 + editor.cursor as u16, 1)
        )?;
        stdout.flush()?;
    }

    write!(
        stdout,
        "{}{}",
        termion::cursor::Goto(1, 1),
        termion::clear::All
    )?;
    editor.save(filepath)?;

    Ok(())
}
