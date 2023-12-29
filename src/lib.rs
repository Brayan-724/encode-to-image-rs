pub mod color;
pub mod decode;
pub mod encode;
pub(crate) mod macros;
pub mod utils;

// Encoders
pub mod chameleon_encoder;
pub mod image_encoder;
pub mod mask_encoder;

pub use color::Color;
