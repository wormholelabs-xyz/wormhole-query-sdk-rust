/// Length: 35
pub const MESSAGE_PREFIX: &[u8] = b"query_response_0000000000000000000|";
pub const QUERY_MESSAGE_LEN: usize = MESSAGE_PREFIX.len() + 32;

pub mod structs;
