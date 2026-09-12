//! Mirrors `net.h4bbo.lisbon.server.mus.streams.MusTypes`.
//!
//! MUS content-type codes. Java exposes these as static `short` fields on the
//! `MusTypes` class; here they are module-level constants, referenced as
//! `MusTypes::<NAME>` (the module is imported as `MusTypes`).

/// No encoded data or additional bytes.
pub const VOID: i16 = 0;
/// 4 bytes in network (not Intel) order.
pub const INTEGER: i16 = 1;
/// 4 bytes with string length, then N bytes of string data, not null
/// terminated, but padded to an even byte boundary.
pub const SYMBOL: i16 = 2;
/// 4 bytes with string length, then N bytes of string data, not null
/// terminated, but padded to an even byte boundary.
pub const STRING: i16 = 3;
/// 4 bytes with data length, then N bytes of binary picture data, padded to an
/// even byte boundary.
pub const PICTURE: i16 = 5;
/// 8 bytes in network (not Intel) order.
pub const FLOAT: i16 = 6;
/// 4 bytes with the number of elements in the list, then N values.
pub const LIST: i16 = 7;
/// 2 numeric values, each may be either an integer or float. First is X, then
/// Y.
pub const POINT: i16 = 8;
/// 4 numeric values, each may be either an integer or float. First is top,
/// left, bottom, then right.
pub const RECT: i16 = 9;
/// 4 bytes with the number of elements in the list, then N pairs of values.
/// The first of each pair is a symbol, then the corresponding value.
pub const PROPLIST: i16 = 10;
/// 4 bytes of binary data.
pub const COLOR: i16 = 18;
/// 16 bytes of binary data.
pub const DATE: i16 = 19;
/// 4 bytes with the data length, then N bytes of binary media data, padded to
/// an even byte boundary.
pub const MEDIA: i16 = 20;
/// 3 floats (8 bytes each, 24 total) in network order.
pub const VECTOR3D: i16 = 22;
/// 16 floats (8 bytes each, 128 total) in network order.
pub const TRANSFORM3D: i16 = 23;
