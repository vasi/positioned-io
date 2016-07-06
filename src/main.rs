extern crate positioned_io;
use positioned_io::{ReadAt, WriteAt, Slice};

fn main() {
    let mut back = [0; 10];
    let mut buf = [0; 10];
    back.as_mut().read_at(0, &mut buf);

    let slice = Slice::new(back.as_mut(), 0, Some(10));
    slice.read_at(0, &mut buf);
}
