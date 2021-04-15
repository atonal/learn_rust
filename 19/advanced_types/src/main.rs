use std::fmt;
use std::io::Error;

// this is in std::io
// type Result<T> = std::result::Result<T, std::io::Error>;
pub trait Write {
    // fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

fn main() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 6;

    println!("x + y = {}", x + y);

    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi!"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    fn bar() -> ! {
        // --snip--
    }
}
