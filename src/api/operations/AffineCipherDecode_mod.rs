// use crate::api::error::Error;
// use super::{Operation, Request};
// use crate::api::macros::create_struct;
//
// create_struct!(AffineCipherDecode);
//
// impl Operation for AffineCipherDecode {
//     fn new(input: Request) -> Box<Self> {
//         Box::new(AffineCipherDecode {
//             name: "Affine Cipher Decode",
//             module: "Cipher",
//             description: Some("The Affine cipher is a type of monoalphabetic substitution cipher. To decrypt, each letter in an alphabet is mapped to its numeric equivalent, decrypted by a mathematical function, and converted back to a letter."),
//             infoURL: Some("https://wikipedia.org/wiki/Affine_cipher"),
//             request: input,
//         })
//     }
//
//     fn run(&self) -> Result<String, Error> {
//         if let Err(e) = self.validate() {
//             return Err(e);
//         }
//
//         Ok(String::new())
//     }
//
//     fn validate(&self) -> Result<(), Error> {
//         let langs
//     }
// }
