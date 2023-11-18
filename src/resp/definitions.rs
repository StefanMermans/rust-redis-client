pub const SIMPLE_STRING: u8 = b'+';
pub const SIMPLE_ERROR: u8 = b'-';
pub const INTEGER: u8 = b':';
pub const BULK_STRING: u8 = b'$';
pub const ARRAY: u8 = b'*';
pub const NULL: u8 = b'_';
pub const BOOLEAN: u8 = b'#';
pub const DOUBLE: u8 = b',';
pub const BIG_NUMBER: u8 = b'(';
pub const BULK_ERROR: u8 = b'!';
pub const VERBATIM_STRING: u8 = b'=';
pub const MAP: u8 = b'%';
pub const SET: u8 = b'~';
pub const PUSH: u8 = b'>';

pub const END: &[u8; 2] = b"\r\n";

