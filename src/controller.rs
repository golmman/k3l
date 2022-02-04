use std::cmp::max;
use std::cmp::min;
use std::io::stdin;
use std::io::Write;
use std::sync::mpsc::sync_channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::SyncSender;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

use termion::color::Fg;
use termion::color::Green;
use termion::color::LightGreen;
use termion::color::Reset;
use termion::event::Key;
use termion::input::TermRead;

use crate::common::frame_string;
use crate::common::Point;
use crate::screen::DefaultScreen;
use crate::state::State;

pub enum TerminalEvent {
    Key(Key),
    Resize,
    Elapse,
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
        self.state.cursor_pos = Point::new(self.screen.cols / 2, self.screen.rows / 2);

        let elapse_sender = self.sender.clone();
        let key_sender = self.sender.clone();
        let resize_sender = self.sender.clone();

        thread::spawn(move || Controller::send_elapse_events(elapse_sender));
        thread::spawn(move || Controller::send_key_events(key_sender));
        thread::spawn(move || Controller::send_resize_events(resize_sender));

        self.draw();

        while self.receive_event() {}
    }

    fn send_elapse_events(sender: SyncSender<TerminalEvent>) {
        let start_instant = Instant::now();

        loop {
            sleep(Duration::from_millis(250));
            let _ = sender.send(TerminalEvent::Elapse);
        }
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
                    Key::Char('H') => self.state.move_cursor_left(),
                    Key::Char('L') => self.state.move_cursor_right(),
                    Key::Char('K') => self.state.move_cursor_up(),
                    Key::Char('J') => self.state.move_cursor_down(),
                    Key::Char('h') => self.state.move_map_right(),
                    Key::Char('l') => self.state.move_map_left(),
                    Key::Char('k') => self.state.move_map_down(),
                    Key::Char('j') => self.state.move_map_up(),
                    _ => {}
                }
            }
            TerminalEvent::Resize => self.screen.resize(),
            TerminalEvent::Elapse => {
                self.state.elapsed_time += 1;
            }
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
        self.draw_map();
        self.draw_debug_info();
        self.draw_cursor();
    }

    fn draw_cursor(&mut self) {
        write!(
            self.screen_buffer,
            "{}{}{}{}",
            termion::cursor::Goto(self.state.cursor_pos.x, self.state.cursor_pos.y),
            Fg(Green),
            "X",
            Fg(Reset),
        )
        .unwrap();
    }

    fn draw_debug_info(&mut self) {
        write!(self.screen_buffer, "{}", termion::cursor::Goto(10, 3),).unwrap();
        write!(
            self.screen_buffer,
            "{:?} - cols: {}, rows: {}, time: {}",
            self.last_key, self.screen.cols, self.screen.rows, self.state.elapsed_time,
        )
        .unwrap();

        write!(self.screen_buffer, "{}", termion::cursor::Goto(10, 4),).unwrap();
        write!(
            self.screen_buffer,
            "map_x: {}, map_y: {}",
            self.state.map_pos.x, self.state.map_pos.y,
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

    fn draw_map(&mut self) {
        let map_x = self.state.map_pos.x;
        let map_y = self.state.map_pos.y;
        let screen_cols = self.screen.cols;
        let screen_rows = self.screen.rows;

        for row in 0..self.state.map.tiles.len() {
            let row = row as u16;
            let full_row = self.state.map.get_row(row);
            let full_row_len = full_row.len() as i16;

            let displayed_row = frame_string(&full_row, map_x, screen_cols);

            if displayed_row.is_empty() {
                continue;
            }

            let goto_x = min(screen_cols, max(1, map_x) as u16) as u16;
            let goto_y = min(screen_rows, max(1, map_y + row as i16) as u16) as u16;

            write!(
                self.screen_buffer,
                "{}{}",
                termion::cursor::Goto(goto_x, goto_y),
                displayed_row,
            )
            .unwrap();
        }
    }
}
