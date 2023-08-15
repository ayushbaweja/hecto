use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {}

impl Editor {
    pub fn run(&self) {
        // writer controls terminal state
        // _stdout owns into_raw_mode return to prevent canonical mode
        let _stdout = stdout().into_raw_mode().unwrap(); 
        loop {
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }

    pub fn default() -> Self { // static method
        Self{}
    }

    fn process_keypress(&self) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Ctrl('q') => panic!("Program end"),
            _ => (),
        }
        Ok(())
    } 
}

fn die(e: std::io::Error) {
    panic!("{}", e);
}

fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() { // shortcut for match with 1 case
            return key;
        }
    }
}