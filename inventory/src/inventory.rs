use crate::artifact::{Arch, Artifact, Os};
use crate::checksum::Digest;
use crate::version::VersionRequirement;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use std::fs;
use std::path::Path;
use std::str::FromStr;

/// Represents an inventory of artifacts.
#[derive(Debug, Serialize, Deserialize)]
pub struct Inventory<V, D> {
    #[serde(bound = "V: Serialize + DeserializeOwned, D: Digest")]
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

    pub fn resolve<R>(&self, os: Os, arch: Arch, requirement: &R) -> Option<&Artifact<V, D>>
    where
        V: Ord,
        R: VersionRequirement<V>,
    {
        self.artifacts
            .iter()
            .filter(|artifact| {
                artifact.os == os
                    && artifact.arch == arch
                    && requirement.satisfies(&artifact.version)
            })
            .max_by_key(|artifact| &artifact.version)
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
pub fn read_inventory_file<V, D>(
    path: impl AsRef<Path>,
) -> Result<Inventory<V, D>, ReadInventoryError>
where
    V: Serialize + DeserializeOwned,
    D: Digest,
{
    toml::from_str(&fs::read_to_string(path)?).map_err(ReadInventoryError::Parse)
}

impl<V, D> FromStr for Inventory<V, D>
where
    V: Serialize + DeserializeOwned,
    D: Digest,
{
    type Err = toml::de::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s)
    }
}

impl<V, D> std::fmt::Display for Inventory<V, D>
where
    V: Serialize + DeserializeOwned,
    D: Digest,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&toml::to_string(self).unwrap())
    }
}

#[cfg(test)]
mod test {
    use crate::artifact::{Arch, Artifact, Os};
    use crate::checksum::tests::BogusDigest;
    use crate::inventory::Inventory;

    #[test]
    fn test_matching_artifact_resolution() {
        let mut inventory = Inventory::new();
        inventory.push(create_artifact("foo", Os::Linux, Arch::Arm64));

        assert_eq!(
            "foo",
            &inventory
                .resolve(Os::Linux, Arch::Arm64, &String::from("foo"))
                .expect("should resolve matching artifact")
                .version,
        );
    }

    #[test]
    fn test_dont_resolve_artifact_with_wrong_arch() {
        let mut inventory = Inventory::new();
        inventory.push(create_artifact("foo", Os::Linux, Arch::Arm64));

        assert!(inventory
            .resolve(Os::Linux, Arch::Amd64, &String::from("foo"))
            .is_none());
    }

    #[test]
    fn test_dont_resolve_artifact_with_wrong_version() {
        let mut inventory = Inventory::new();
        inventory.push(create_artifact("foo", Os::Linux, Arch::Arm64));

        assert!(inventory
            .resolve(Os::Linux, Arch::Arm64, &String::from("bar"))
            .is_none());
    }

    fn create_artifact(version: &str, os: Os, arch: Arch) -> Artifact<String, BogusDigest> {
        Artifact {
            version: String::from(version),
            os,
            arch,
            url: "https://example.com".to_string(),
            checksum: BogusDigest::checksum("cafebabe"),
        }
    }
}
