use crate::artifact::{Arch, Artifact, Os};
use crate::checksum::Name;
use crate::version::VersionRequirement;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fs;

/// Represents an inventory of artifacts.
#[derive(Debug, Serialize, Deserialize)]
pub struct Inventory<V, D> {
    #[serde(bound = "V: Serialize + DeserializeOwned, D: Name")]
    pub artifacts: Vec<Artifact<V, D>>,
}

impl<V, D> Default for Inventory<V, D> {
    fn default() -> Self {
        Self { artifacts: vec![] }
    }
}

impl<V, D> Inventory<V, D> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, artifact: Artifact<V, D>) {
        self.artifacts.push(artifact);
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ReadInventoryError {
    #[error("Couldn't read inventory file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Couldn't parse inventory toml: {0}")]
    Parse(#[from] toml::de::Error),
}

/// Reads a TOML-formatted file to an `Inventory<V, D>`.
///
/// # Errors
///
/// Will return an Err if the file is missing, not readable, or if the
/// file contents is not formatted properly.
pub fn read_inventory_file<V, D>(path: &str) -> Result<Inventory<V, D>, ReadInventoryError>
where
    V: Serialize + DeserializeOwned,
    D: Name,
{
    toml::from_str(&fs::read_to_string(path)?).map_err(ReadInventoryError::Parse)
}

/// Find the first artifact that satisfies a `VersionRequirement<V>` for
/// the specified OS and arch.
pub fn resolve<'a, V, D, R>(
    artifacts: &'a [Artifact<V, D>],
    os: Os,
    arch: Arch,
    requirement: &'a R,
) -> Option<&'a Artifact<V, D>>
where
    R: VersionRequirement<V>,
{
    artifacts
        .iter()
        .filter(|artifact| artifact.os == os && artifact.arch == arch)
        .find(|artifact| requirement.satisfies(&artifact.version))
}

#[cfg(test)]
mod test {
    use crate::artifact::{Arch, Artifact, Os};
    use crate::checksum::Checksum;
    use crate::inventory::resolve;

    #[test]
    fn test_matching_artifact_resolution() {
        assert_eq!(
            "foo",
            &resolve(
                &[create_artifact("foo", Os::Linux, Arch::Arm64)],
                Os::Linux,
                Arch::Arm64,
                &String::from("foo")
            )
            .expect("should resolve matching artifact")
            .version,
        );
    }

    #[test]
    fn test_dont_resolve_artifact_with_wrong_arch() {
        assert!(resolve(
            &[create_artifact("foo", Os::Linux, Arch::Arm64)],
            Os::Linux,
            Arch::Amd64,
            &String::from("foo")
        )
        .is_none());
    }

    #[test]
    fn test_dont_resolve_artifact_with_wrong_version() {
        assert!(resolve(
            &[create_artifact("foo", Os::Linux, Arch::Arm64)],
            Os::Linux,
            Arch::Arm64,
            &String::from("bar")
        )
        .is_none());
    }

    fn create_artifact(version: &str, os: Os, arch: Arch) -> Artifact<String, String> {
        Artifact::<String, String> {
            version: String::from(version),
            os,
            arch,
            url: "https://example.com".to_string(),
            checksum: Checksum::try_from("aaaa".to_string()).unwrap(),
        }
    }
}
