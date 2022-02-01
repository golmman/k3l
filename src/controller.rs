use std::io::stdin;
use std::io::Write;
use std::sync::mpsc::sync_channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::SyncSender;
use std::thread;

use termion::event::Key;
use termion::input::TermRead;

use crate::screen::DefaultScreen;

pub enum TerminalEvent {
    Key(Key),
    Resize,
}

pub struct Controller {
    screen: DefaultScreen,

    sender: SyncSender<TerminalEvent>,
    receiver: Receiver<TerminalEvent>,
}

impl Controller {
    pub fn new(screen: DefaultScreen) -> Self {
        let (sender, receiver) = sync_channel::<TerminalEvent>(1024);
        Self {
            screen,
            sender,
            receiver,
        }
    }

    pub fn run(&mut self) {
        let key_sender = self.sender.clone();
        let resize_sender = self.sender.clone();

        thread::spawn(move || Controller::send_key_events(key_sender));
        thread::spawn(move || Controller::send_resize_events(resize_sender));

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
        let mut k: Option<Key> = None;
        let event = self.receiver.recv().unwrap();

        let mut screen_buffer = Vec::<u8>::new();

        write!(
            screen_buffer,
            "{}{}",
            termion::cursor::Goto(10, 3),
            termion::clear::All,
        )
        .unwrap();

        match event {
            TerminalEvent::Key(key) => {
                k = Some(key);
                match key {
                    termion::event::Key::Char('q') => return false,
                    _ => {}
                }
            }
            TerminalEvent::Resize => self.screen.resize(),
        }

        writeln!(
            screen_buffer,
            "{k:?} - cols: {}, rows: {}",
            self.screen.cols, self.screen.rows
        )
        .unwrap();

        screen_buffer.flush().unwrap();

        // double buffering
        self.screen
            .write_all(&screen_buffer)
            .unwrap();
        self.screen.flush().unwrap();

        true
    }
}
