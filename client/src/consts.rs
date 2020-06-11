use std::mem::size_of;

// Message consts
pub const MAX_MESSAGE_LENGTH: usize = 501;
pub const MAX_MESSAGE_TEXT_LENGTH: usize = MAX_MESSAGE_LENGTH - size_of::<u32>() - size_of::<usize>();
pub const MESSAGE_WARNING_LENGHT: usize = (MAX_MESSAGE_TEXT_LENGTH as f64 * 0.90) as usize;
pub const MESSAGE_ERROR_LENGHT: usize = (MAX_MESSAGE_TEXT_LENGTH as f64 * 0.95) as usize;
