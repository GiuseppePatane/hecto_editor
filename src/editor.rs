use crate::document::Document;
use crate::row::Row;
use crate::terminal::Terminal;
use std::env;
use std::io::Error;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");
#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    document: Document,
}
impl Editor {
    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect(); // get the arguments passed to the editor
        let document = if args.len() > 1 {
            // if the user passes a file name as an argument, open the file
            let file_name = &args[1]; // get the file name
            Document::open(&file_name).unwrap_or_default() // open the file
        } else {
            Document::default() // else create a new document
        };
        Self {
            should_quit: false, // if  the user presses ctrl + q, this will be set to true
            terminal: Terminal::default().expect("Failed to initialize terminal"), // initialize the terminal
            cursor_position: Position::default(),
            document,
        }
    }
    /// This function will draw the welcome message  for the editor
    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Hecto editor -- version {}", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }

    /// This function will run the editor
    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
            if self.should_quit {
                break;
            }
        }
    }
    /// function to process the key pressed by the user
    fn process_keypress(&mut self) -> Result<(), Error> {
        let pressed_key = Terminal::read_key()?; // read the key pressed by the user
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up
            | Key::Down
            | Key::Left
            | Key::Right
            | Key::PageUp
            | Key::PageDown
            | Key::End
            | Key::Home => self.move_cursor(pressed_key),
            _ => (),
        }
        Ok(())
    }
    /// function to refresh the screen
    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::cursor_hide();
        Terminal::clear_screen();
        Terminal::cursor_position(&Position::default());
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&self.cursor_position);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    pub fn draw_raw(&self, row: &Row) {
        let start = 0;
        let end = self.terminal.size().width as usize;
        let row = row.render(start, end);
        println!("{}\r", row)
    }

    /// function to draw the rows
    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height - 1 {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(terminal_row as usize) {
                self.draw_raw(row);
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message();
            } else if terminal_row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }
    /// function to move the cursor on the screen
    fn move_cursor(&mut self, key: Key) {
        let Position { mut y, mut x } = self.cursor_position;
        let size = self.terminal.size();
        let height = size.height.saturating_sub(1) as usize;
        let width = size.width.saturating_sub(1) as usize;
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1);
                }
            }
            Key::PageUp => y = 0,
            Key::PageDown => y = height,
            Key::Home => x = 0,
            Key::End => x = width,
            _ => (),
        }
        self.cursor_position = Position { x, y }
    }
}

/// function to handle the error
fn die(e: Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
