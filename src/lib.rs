mod operations;

pub use operations::*;
pub use utils::{Operation, Operations};
use serde::{Deserialize, Serialize};

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
