use std::io::stdin;
use std::io::Write;
use std::sync::mpsc::sync_channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::SyncSender;
use std::thread;

use termion::event::Key;
use termion::input::TermRead;

use crate::screen::DefaultScreen;
use crate::state::State;

pub enum TerminalEvent {
    Key(Key),
    Resize,
}

pub struct Controller {
    screen: DefaultScreen,
    state: State,

    screen_buffer: Vec<u8>,
    last_key: Option<Key>,
    sender: SyncSender<TerminalEvent>,
    receiver: Receiver<TerminalEvent>,
}

impl Controller {
    pub fn new(screen: DefaultScreen, state: State) -> Self {
        let (sender, receiver) = sync_channel::<TerminalEvent>(1024);
        let last_key = None;
        let screen_buffer = Vec::<u8>::new();

        Self {
            screen,
            state,

            screen_buffer,
            last_key,
            sender,
            receiver,
        }
    }

    pub fn run(&mut self) {
        let key_sender = self.sender.clone();
        let resize_sender = self.sender.clone();

        thread::spawn(move || Controller::send_key_events(key_sender));
        thread::spawn(move || Controller::send_resize_events(resize_sender));

        self.draw();

        while self.receive_event() {}
    }

    fn send_key_events(sender: SyncSender<TerminalEvent>) {
        let stdin = stdin();

        for key in stdin.keys().flatten() {
            let _ = sender.send(TerminalEvent::Key(key));
        }
    }

    fn send_resize_events(sync_sender: SyncSender<TerminalEvent>) {
        let _ = unsafe {
            signal_hook::low_level::register(signal_hook::consts::SIGWINCH, move || {
                sync_sender
                    .send(TerminalEvent::Resize)
                    .unwrap();
            })
        };
    }

    fn receive_event(&mut self) -> bool {
        let event = self.receiver.recv().unwrap();

        match event {
            TerminalEvent::Key(key) => {
                self.last_key = Some(key);
                match key {
                    Key::Char('q') => return false,
                    Key::Char('h') => self.state.move_cursor_left(),
                    Key::Char('l') => self.state.move_cursor_right(),
                    Key::Char('k') => self.state.move_cursor_up(),
                    Key::Char('j') => self.state.move_cursor_down(),
                    _ => {}
                }
            }
            TerminalEvent::Resize => self.screen.resize(),
        }

        self.clear_screen();

        self.draw();

        self.screen_buffer.flush().unwrap();

        // double buffering
        self.screen
            .write_all(&self.screen_buffer)
            .unwrap();
        self.screen.flush().unwrap();

        true
    }

    pub fn clear_screen(&mut self) {
        self.screen_buffer.clear();
        write!(self.screen_buffer, "{}", termion::clear::All,).unwrap();
    }

    pub fn draw(&mut self) {
        self.draw_floor();
        self.draw_debug_info();
        self.draw_cursor();
    }

    fn draw_cursor(&mut self) {
        write!(
            self.screen_buffer,
            "{}{}",
            termion::cursor::Goto(self.state.cursor_pos.x, self.state.cursor_pos.y),
            "X",
        )
        .unwrap();
    }

    fn draw_debug_info(&mut self) {
        write!(self.screen_buffer, "{}", termion::cursor::Goto(10, 3),).unwrap();

        writeln!(
            self.screen_buffer,
            "{:?} - cols: {}, rows: {}",
            self.last_key, self.screen.cols, self.screen.rows
        )
        .unwrap();
    }

    fn draw_floor(&mut self) {
        for y in 0..self.screen.rows {
            write!(
                self.screen_buffer,
                "{}{}",
                termion::cursor::Goto(1, y + 1),
                ".".repeat(self.screen.cols.into())
            )
            .unwrap();
        }
    }
}
