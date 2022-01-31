use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::thread;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub enum Event {
    Key(Key),
}

pub fn send_key_events(sender: SyncSender<Event>) {
    let stdin = stdin();

    for key in stdin.keys() {
        if let Ok(key) = key {
            let _ = sender.send(Event::Key(key));
        }
    }
}

pub fn receive_event(receiver: &Receiver<Event>, screen: &mut dyn Write) -> bool {
    let event = receiver.recv().unwrap();

    write!(screen, "{}", termion::clear::All);
    match event {
        Event::Key(key) => write!(screen, "{key:?}\n").unwrap(),
    }

    screen.flush().unwrap();

    true
}

fn drop(screen: &mut dyn Write) {
    write!(
        screen,
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Show,
    )
    .unwrap();
}

fn main() {
    println!("Hello, world!");

    let mut screen = stdout().into_raw_mode().unwrap();

    //drop(&mut screen);
    //return;


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

    let sender1 = sender.clone();
    thread::spawn(move || send_key_events(sender1));

    while receive_event(&receiver, &mut screen) {}
}
