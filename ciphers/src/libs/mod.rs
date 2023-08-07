mod affine_trait;
mod base64;
mod switch_case_trait;
mod vigenere_trait;

pub use affine_trait::AffineCipher;
pub use base64::{from_base64, to_base64};
pub use switch_case_trait::SwitchCase;
pub use vigenere_trait::VigenereCipher;
