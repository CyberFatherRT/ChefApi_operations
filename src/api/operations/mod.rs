pub mod A1Z26CipherDecode;

use derivative::Derivative;

#[derive(Derivative)]
pub struct Operation {
    #[derivative(Default(value = "en"))]
    pub(crate) lang: String,
    pub(crate) params: Vec<String>,
    pub(crate) input: String,
}
