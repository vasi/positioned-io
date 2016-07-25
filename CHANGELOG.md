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
