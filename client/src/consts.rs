use std::mem::size_of;

// Message consts
pub const MAX_MESSAGE_LENGTH: usize = 501;
pub const MAX_MESSAGE_TEXT_LENGTH: usize = MAX_MESSAGE_LENGTH - size_of::<u32>() - size_of::<usize>();
pub const MESSAGE_WARNING_LENGHT: usize = (MAX_MESSAGE_TEXT_LENGTH as f64 * 0.90) as usize;
pub const MESSAGE_ERROR_LENGHT: usize = (MAX_MESSAGE_TEXT_LENGTH as f64 * 0.95) as usize;

// Conf dir
pub const CONFIG_DIR: &'static str = "barium";

// Key Size
pub const KEY_SIZE: usize = 4096;

// Default port
pub const DEFAULT_PORT: u16 = 13337;

// TCP timeout in seconds
pub const TCP_TIMEOUT: u64 = 10;
