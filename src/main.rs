extern crate termion;

use std::fs;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

#[derive(PartialEq)]
enum Mode {
    Normal,
    Insert,
}

#[derive(PartialEq)]
struct Editor {
    mode: Mode,
    buffer: String,
    cursor_x: u16,
    cursor_y: u16,
    screen_rows: u16,
    inserting_file: bool,
    file_name: Option<String>, // new field to store the filename
}


impl Editor {
    fn new(screen_rows: u16, filename: Option<String>) -> Editor {
        let (buffer, inserting_file) = match filename {
            Some(ref filename) => {
                let file_contents = fs::read_to_string(&filename).unwrap_or_default();
                (file_contents, true)
            }
            None => (String::new(), false),
        };

        Editor {
            mode: Mode::Insert,
            buffer,
            cursor_x: 1,
            cursor_y: 1,
            screen_rows,
            inserting_file,
            file_name: filename,
        }
    }

    fn save(&self) {
        if let Some(ref file_name) = self.file_name {
            fs::write(file_name, &self.buffer).expect("Failed to write file");
        }
    }

    fn cursor_left(&mut self) {
        if self.cursor_x > 1 {
            self.cursor_x -= 1;
        }
    }

    fn cursor_right(&mut self) {
        if self.cursor_x < self.buffer_at_cursor().len() as u16 + 1 {
            self.cursor_x += 1;
        }
    }

    fn cursor_home(&mut self) {
        self.cursor_x = 1;
    }

    fn cursor_end(&mut self) {
        self.cursor_x = self.buffer_at_cursor().len() as u16 + 1;
    }

    fn buffer_at_cursor(&self) -> &str {
        let mut lines = self.buffer.lines();
        let mut line = lines.nth((self.cursor_y - 1) as usize).unwrap_or("");
        if line.len() < self.cursor_x as usize - 1 {
            line = "";
        }
        &line[(self.cursor_x - 1) as usize..]
    }

    fn draw_screen(&self) {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        println!("{}", self.buffer);
        print!("{}{}", termion::cursor::Goto(self.cursor_x, self.cursor_y), termion::cursor::Show);
        stdout().flush().unwrap();
    }

    fn run(&mut self) {
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();
        self.draw_screen();

        for c in stdin.keys() {
            match c.unwrap() {
                Key::Esc => {
                    if self.mode == Mode::Insert {
                        self.mode = Mode::Normal;
                    } else {
                        self.mode = Mode::Insert;
                    }
                    print!("{}", termion::cursor::Hide);
                }

                Key::Char(c) => {
                    match self.mode {
                        Mode::Insert => {
                            self.buffer.insert((self.cursor_x - 1) as usize, c);
                            self.cursor_x += 1;
                            self.draw_screen();
                        }

                        Mode::Normal => {
                            match c {
                                'q' => {
                                    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Show).unwrap();
                                    drop(stdout);
                                    std::process::exit(0);
                                }
                                _ => {}
                            }
                        }
                    }
                }
                
                // handle other events
                Key::Backspace => {
                    if self.mode == Mode::Insert && !self.buffer.is_empty() {
                        let cursor_pos = (self.cursor_y - 1) * self.screen_rows + (self.cursor_x - 1);
                        if cursor_pos > 0 {
                            self.buffer.remove((cursor_pos - 1).into());
                            self.cursor_x -= 1;
                        }
                        self.draw_screen();
                    }
                }

                Key::Ctrl('c') => {
                    print!("{}", termion::cursor::Show);
                    drop(stdout);
                    std::process::exit(0);
                }
                
                Key::Ctrl('s') => {
                    self.save();
                }

                Key::Left => {
                    self.cursor_left()
                }
                Key::Right => {
                    self.cursor_right()
                }
                Key::Home => {
                    self.cursor_home()
                }
                Key::End => {
                    self.cursor_end()
                }
                
                _ => {}
            }
        }

        drop(stdout);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = args.get(1).cloned(); // the first argument is the filename

    let mut editor = Editor::new(termion::terminal_size().unwrap().1, filename);
    editor.run();
}
