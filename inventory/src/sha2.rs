use crate::checksum::Digest;
use sha2::{Sha256, Sha512};

impl Digest for Sha256 {
    fn name_compatible(name: &str) -> bool {
        name == "sha256"
    }

    fn length() -> usize {
        Self::output_size()
    }
}

impl Digest for Sha512 {
    fn name_compatible(name: &str) -> bool {
        name == "sha512"
    }

    fn length() -> usize {
        Self::output_size()
    }
}
