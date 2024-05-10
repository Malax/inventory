use crate::checksum::{ChecksumSize, DigestName};
use sha2::{Digest, Sha256, Sha512};

impl DigestName for Sha256 {
    fn name() -> String {
        String::from("sha256")
    }
}

impl DigestName for Sha512 {
    fn name() -> String {
        String::from("sha512")
    }
}

impl ChecksumSize for Sha256 {
    fn checksum_size() -> usize {
        Self::output_size()
    }
}

impl ChecksumSize for Sha512 {
    fn checksum_size() -> usize {
        Self::output_size()
    }
}
