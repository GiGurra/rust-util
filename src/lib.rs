pub mod file;
pub mod k8s;
pub mod shell;
pub mod string;
pub mod logging;
pub mod crypt;

pub use serde_yaml as serde_yaml;
pub use serde as serde;
pub use regex as regex;
pub use log as log;
pub use simple_logger as simple_logger;
pub use itertools as itertools;
pub use clap as clap;
pub use aes_gcm as aes_gcm;
pub use rand as rand;
pub use rand_chacha as rand_chacha;
pub use hex as hex;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
