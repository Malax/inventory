use crate::checksum::{ChecksumSize, DigestName};
use sha2::digest::OutputSizeUser;
use sha2::{Digest, Sha256, Sha512};

impl DigestName for Sha256 {
    fn name_compatible(name: &str) -> bool {
        name == "sha256"
    }
}

impl DigestName for Sha512 {
    fn name_compatible(name: &str) -> bool {
        name == "sha512"
    }
}

impl<T> ChecksumSize for T
where
    T: OutputSizeUser,
{
    fn checksum_size() -> usize {
        Self::output_size()
    }
}
