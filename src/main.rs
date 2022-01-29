use std::sync::mpsc::sync_channel;

enum Event {
    Key,
}

fn main() {
    let (sender, receiver) = sync_channel::<Event>(1024);

    println!("Hello, world!");
}
