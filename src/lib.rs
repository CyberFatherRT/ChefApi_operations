mod operations;

pub use operations::*;
use serde::{Deserialize, Serialize};
pub use utils::{Operation, Operations};

pub fn run_operations<'a, I, O>(
    operations: impl Operation<'a, I, O>,
    request: &str,
) -> Result<O, String>
where
    I: Deserialize<'a>,
    O: Serialize,
{
    operations.do_black_magic(request)
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Hex(String),
    Base64(String),
    Uint8Array(Vec<u8>),
}
