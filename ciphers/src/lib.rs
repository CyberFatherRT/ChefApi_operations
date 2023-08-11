// pub mod libs;
mod macros;
mod operations;
pub mod traits;
pub mod utils;

pub use operations::argon2_mod::{Argon2, Argon2Info};

use serde::{Deserialize, Serialize};

pub trait Operation<'a, O>
where
    O: Deserialize<'a>,
{
    fn new(request: String) -> Self;
    fn run(&self) -> Result<String, String>;
    fn validate(&self, request: &'a str) -> Result<O, String> {
        self.deserialize(request)
    }

    fn deserialize(&self, request: &'a str) -> Result<O, String> {
        serde_json::from_str(request).map_err(|err| match err.to_string() {
            err if err.starts_with("unknown") || err.starts_with("missing") => {
                err.split(" at line ").next().unwrap().to_string()
            }
            err => err,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub enum Operations {
    /// Argon2 is a key derivation function that was selected as the winner of the Password Hashing Competition in July 2015. It was designed by Alex Biryukov, Daniel Dinu, and Dmitry Khovratovich from the University of Luxembourg.
    /// <br/><br/>
    /// For more information go to this site - https://wikipedia.org/wiki/Argon2
    /// <br/><br/>
    ///
    /// ### How to use
    /// \
    /// Send POST requests to /api/Argon2 with your data using json payload with this structure
    /// ``` json
    /// {
    ///     "input": string,
    ///     "params": {
    ///         "salt": string,
    ///         "iterations": u32,
    ///         "parallelism": u32,
    ///         "hash_length": u32,
    ///         "argon2_type": Argon2Type,
    ///         "output_format": OutputFormat,
    ///         "memory": u32
    ///     }
    /// }
    /// ```
    /// #### where
    ///     - u32 is unsigned 32-bit integer
    ///     - Argon2Type is one of "Argon2i", "Argon2d", "Argon2id"
    ///     - OutputFormat is one of "Encoded", "Hex", "Raw"
    /// <br/><br/>
    ///
    /// ### Server response have two possible formats
    ///
    /// #### &nbsp;&nbsp;&nbsp;&nbsp; Ok variant
    /// ``` json
    /// { "Ok": `some answer` }
    /// ```
    /// #### &nbsp;&nbsp;&nbsp;&nbsp; Error variant
    /// ``` json
    /// { "Err": `error message` }
    /// ```
    /// ### Examples
    /// <br><br/>
    /// #### №1
    /// ``` http
    /// POST /api/Argon2
    /// content_type: application/json; charset=utf-8
    ///
    /// {
    ///     "input": "hello",
    ///     "params": {
    ///         "salt": "somesalt",
    ///         "iterations": 3,
    ///         "parallelism": 1,
    ///         "hash_length": 32,
    ///         "argon2_type": "Argon2i",
    ///         "output_format": "Encoded",
    ///         "memory": 4096
    ///     }
    /// }
    /// ```
    /// ```http
    /// HTTP/1.1 200 Ok
    /// {
    ///   "Ok": "$argon2i$v=19$m=4096,t=3,p=1$c29tZXNhbHQ$WVDOfucSPAey3UEzzqLtBwRbGS83pTyIPLXgjhKfgrY"
    /// }
    /// ```
    /// #### №2
    /// ``` http
    /// POST /api/Argon2
    /// content_type: application/json; charset=utf-8
    ///
    /// {
    ///     "input": "Привет, Мир!",
    ///     "params": {
    ///         "salt": "новая соль",
    ///         "iterations": 6,
    ///         "parallelism": 1,
    ///         "hash_length": 34,
    ///         "argon2_type": "Argon2id",
    ///         "output_format": "Hex",
    ///         "memory": 8096
    ///     }
    /// }
    /// ```
    /// ```http
    /// {
    ///   "Ok": "eb4140b78ed1c4fcd736c1b73cdf555ba244371ff53971e53823e411aeefbd60751d"
    /// }
    /// ```
    /// #### №3
    /// ``` http
    /// POST /api/Argon2
    /// content_type: application/json; charset=utf-8
    ///
    /// {
    ///     "input": "error",
    ///     "params": {
    ///         "salt": "missing iterations parameter",
    ///         "parallelism": 1,
    ///         "hash_length": 34,
    ///         "argon2_type": "Argon2id",
    ///         "output_format": "Hex",
    ///         "memory": 8096
    ///     }
    /// }
    /// ```
    /// ```http
    /// HTTP/1.1 400 Bad Request
    /// {
    ///   "Err": "missing field `iterations`"
    /// }
    /// ```
    Argon2,
}
