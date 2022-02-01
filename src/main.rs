use std::sync::mpsc::sync_channel;
use std::thread;

use crate::controller::receive_event;
use crate::controller::send_key_events;
use crate::controller::send_resize_events;
use crate::controller::TerminalEvent;
use crate::screen::Screen;

mod controller;
mod screen;

fn main() {
    println!("Hello, world!");

    let mut screen = Screen::new();

    let (sender, receiver) = sync_channel::<TerminalEvent>(1024);

    let key_sender = sender.clone();
    let resize_sender = sender.clone();
    thread::spawn(move || send_key_events(key_sender));
    thread::spawn(move || send_resize_events(resize_sender));

    while receive_event(&receiver, &mut screen) {}
}
