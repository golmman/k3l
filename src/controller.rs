use std::io::stdin;
use std::io::Write;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::SyncSender;

use termion::event::Key;
use termion::input::TermRead;

use crate::screen::Screen;

pub enum TerminalEvent {
    Key(Key),
    Resize,
}

pub fn send_key_events(sender: SyncSender<TerminalEvent>) {
    let stdin = stdin();

    for key in stdin.keys().flatten() {
        let _ = sender.send(TerminalEvent::Key(key));
    }
}

pub fn send_resize_events(sync_sender: SyncSender<TerminalEvent>) {
    let _ = unsafe {
        signal_hook::low_level::register(signal_hook::consts::SIGWINCH, move || {
            sync_sender
                .send(TerminalEvent::Resize)
                .unwrap();
        })
    };
}

pub fn receive_event<W: Write>(receiver: &Receiver<TerminalEvent>, screen: &mut Screen<W>) -> bool {
    let event = receiver.recv().unwrap();

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
            writeln!(
                screen_buffer,
                "{key:?} - cols: {}, rows: {}",
                screen.cols, screen.rows
            )
            .unwrap();

            match key {
                termion::event::Key::Char('q') => return false,
                _ => {}
            }
        }
        TerminalEvent::Resize => {
            let (cols, rows) = termion::terminal_size().unwrap();
            screen.cols = cols;
            screen.rows = rows;
            writeln!(
                screen_buffer,
                "cols: {}, rows: {}",
                screen.cols, screen.rows
            )
            .unwrap();
        }
    }

    screen_buffer.flush().unwrap();

    // double buffering
    screen
        .write_all(&screen_buffer)
        .unwrap();
    screen.flush().unwrap();

    true
}
