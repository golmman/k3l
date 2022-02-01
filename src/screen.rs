use std::io::stdout;
use std::io::Stdout;
use std::io::Write;

use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

pub struct Screen<W: Write> {
    pub buffer: W,
    pub cols: u16,
    pub rows: u16,
}

impl Screen<RawTerminal<Stdout>> {
    pub fn new() -> Self {
        Screen::from(stdout().into_raw_mode().unwrap())
    }
}

impl<W: Write> From<W> for Screen<W> {
    fn from(mut buffer: W) -> Self {
        write!(
            buffer,
            "{}{}{}",
            termion::cursor::Hide,
            termion::cursor::Goto(1, 1),
            termion::clear::All,
        )
        .unwrap();

        buffer.flush().unwrap();

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

        self.flush().unwrap();
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
