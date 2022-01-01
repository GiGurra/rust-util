pub mod file;
pub mod shell;
pub mod string;
pub mod logging;
pub mod crypt;

pub use serde_yaml;
pub use serde;
pub use regex;
pub use log;
pub use simple_logger;
pub use itertools;
pub use clap;
pub use aes_gcm;
pub use rand;
pub use rand_chacha;
pub use hex;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
