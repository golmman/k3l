use std::io::stdin;
use std::sync::mpsc::sync_channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::SyncSender;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use termion::event::Key;
use termion::input::TermRead;

use crate::common::Point;
use crate::renderer::Renderer;
use crate::state::State;

pub enum TerminalEvent {
    Key(Key),
    Resize,
    Elapse,
}

pub struct Controller {
    last_key: Option<Key>,
    receiver: Receiver<TerminalEvent>,
    renderer: Renderer,
    sender: SyncSender<TerminalEvent>,
    state: State,
}

impl Controller {
    pub fn new() -> Self {
        let (sender, receiver) = sync_channel::<TerminalEvent>(1024);
        let last_key = None;
        let renderer = Renderer::new();
        let state = State::new();

        Self {
            last_key,
            receiver,
            renderer,
            sender,
            state,
        }
    }

    pub fn run(&mut self) {
        self.state.cursor_pos =
            Point::new(self.renderer.screen.cols / 2, self.renderer.screen.rows / 2);

        let elapse_sender = self.sender.clone();
        let key_sender = self.sender.clone();
        let resize_sender = self.sender.clone();

        thread::spawn(move || Controller::send_elapse_events(elapse_sender));
        thread::spawn(move || Controller::send_key_events(key_sender));
        thread::spawn(move || Controller::send_resize_events(resize_sender));

        self.renderer.draw(&self.state);

        while self.receive_event() {}
    }

    fn send_elapse_events(sender: SyncSender<TerminalEvent>) {
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
            TerminalEvent::Resize => self.renderer.screen.resize(),
            TerminalEvent::Elapse => {
                self.state.elapsed_time += 1;
            }
        }

        self.renderer.draw(&self.state);

        true
    }
}
