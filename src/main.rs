use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::thread;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Screen<W: Write> {
    buffer: W,
    cols: u16,
    rows: u16,
}

impl<W: Write> Screen<W> {
    pub fn new(mut buffer: W) -> Self {
        write!(
            buffer,
            "{}{}{}",
            termion::cursor::Hide,
            termion::cursor::Goto(1, 1),
            termion::clear::All,
        )
        .unwrap();

        Self {
            buffer,
            cols: 0,
            rows: 0,
        }
    }
}

impl<W: Write> Drop for Screen<W> {
    fn drop(&mut self) {
        write!(
            self,
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Show,
        )
        .unwrap();
    }
}

impl<W: Write> Write for Screen<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.buffer.flush()
    }
}

pub enum Event {
    Key(Key),
    Resize,
}

pub fn send_key_events(sender: SyncSender<Event>) {
    let stdin = stdin();

    for key in stdin.keys().flatten() {
        let _ = sender.send(Event::Key(key));
    }
}

pub fn send_resize_events(sync_sender: SyncSender<Event>) {
    let _ = unsafe {
        signal_hook::low_level::register(signal_hook::consts::SIGWINCH, move || {
            sync_sender.send(Event::Resize).unwrap();
        })
    };
}

pub fn receive_event<W: Write>(receiver: &Receiver<Event>, screen: &mut Screen<W>) -> bool {
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
        Event::Key(key) => {
            writeln!(screen_buffer, "{key:?} - cols: {}, rows: {}", screen.cols, screen.rows).unwrap();

            match key {
                termion::event::Key::Char('q') => return false,
                _ => {}
            }
        }
        Event::Resize => {
            let (cols, rows) = termion::terminal_size().unwrap();
            screen.cols = cols;
            screen.rows = rows;
            writeln!(screen_buffer, "cols: {}, rows: {}", screen.cols, screen.rows).unwrap();
        }
    }

    screen_buffer.flush().unwrap();

    // double buffering
    screen.write_all(&screen_buffer).unwrap();
    screen.flush().unwrap();

    true
}

fn main() {
    println!("Hello, world!");

    let mut screen = Screen::new(stdout().into_raw_mode().unwrap());

    write!(
        screen,
        "{}{}{}",
        termion::cursor::Hide,
        termion::cursor::Goto(1, 1),
        termion::clear::All,
    )
    .unwrap();
    screen.flush().unwrap();

    let (sender, receiver) = sync_channel::<Event>(1024);

    let key_sender = sender.clone();
    let resize_sender = sender.clone();
    thread::spawn(move || send_key_events(key_sender));
    thread::spawn(move || send_resize_events(resize_sender));

    while receive_event(&receiver, &mut screen) {}
}
