# [0.3.0] - 2018-09-12

* Renamed `{Read,Write}BytesExt` to `{Read,Write}BytesAtExt` to avoid overlap
  with `byteorder`.
* `{Read,Write}Int` and `{Read,Write}IntAt` are now inherent methods of
  `ByteIo`.
* Removed `Deref` and `DerefMut` implementations for `ByteIo`.
  Use `ByteIo::as_inner()` and `ByteIo::as_inner_mut()` instead.
* Removed `Deref` and `DerefMut` for `SizeCursor`. `Cursor` methods are now
  also implemented on `SizeCursor`. Use `SizeCursor::as_cursor()` or
  `SizeCursor::as_cursor_mut()` to borrow the underlying `Cursor`.
* `Cursor::get_ref()` renamed to `Cursor::as_inner()`, `Cursor::get_mut()`
  renamed to `Cursor::as_inner_mut()`.
* Fixed `WriteAt for File` on Windows: Writes were not working at all.
* Fixed `ReadAt for File` on Windows: Positioned reads were moving the
  file cursor. The new implementation is much slower but no londer modifies
  the read position.
* Various methods are now inlinable across crate boundaries.


# [0.2.2] - 2016-07-24

* At 8-bit operations to integer read/write traits.


# [0.2.1] - 2016-07-12

* Remove dependency on nix.


# [0.2.0] - 2016-07-11

* Add traits ReadInt, ReadIntAt, WriteInt, WriteIntAt to describe behaviour of
  ByteIo.
* Make ByteIo implement Read, ReadAt, Write, WriteAt, so that it's more useful
  as a trait object.
* Add a changelog.


# [0.1.0] - 2016-07-06

Initial release.

* ReadAt, WriteAt, Size traits
* Implementations for:
  * Files (Unix and Windows)
  * Arrays
  * Vectors
  * References
* byteorder functionality
  * Positioned byte-order extensions ReadBytesExt, WriteBytesExt
  * ByteIo adapter, to make ByteOrder type parameter implicit
* Cursor, to turn a ReadAt/WriteAt into a Read/Write
* Slices, to turn a ReadAt/WriteAt into a smaller ReadAt/WriteAt
* Documentation
* Basic integration tests
