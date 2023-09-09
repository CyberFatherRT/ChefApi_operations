mod operations;

use serde::{Deserialize, Serialize};
pub use operations::*;
use utils::Operation;

pub fn run_operations<'a, I, O>(operations: impl Operation<'a, I, O>, request: &str) -> Result<O, String>
    where
        I: Deserialize<'a>,
        O: Serialize,
{
    operations.do_black_magic(request)
}
