## positioned-io

This crate allows you to specify an offset for reads and writes, without changing the current
position in a file. This is similar to [`pread()` and `pwrite()`][pread] in C.

The major advantages of this type of I/O are:

* You don't need to seek before doing a random-access read or write, which is convenient.
* Reads don't modify the file at all, so don't require mutability.

[pread]: http://man7.org/linux/man-pages/man2/pread.2.html

[![Build Status](https://travis-ci.org/vasi/positioned-io.svg?branch=master)](https://travis-ci.org/vasi/positioned-io)
[![Crates.io](https://img.shields.io/crates/v/positioned-io.svg?maxAge=2592000)]()

### Example

Read the fifth 512-byte sector of a file:

```rust
use positioned_io::ReadAt;

// Note that file does not need to be mut!
let file = try!(File::open("foo.data"));
let mut buf = vec![0; 512];
let bytes_read = try!(file.read_at(2048, &mut buf));
```

### Documentation

http://vasi.github.io/positioned-io/positioned_io/

### Usage

TODO
