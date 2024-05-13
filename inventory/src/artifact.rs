use crate::checksum::{Checksum, Digest};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Artifact<V, D> {
    #[serde(bound = "V: Serialize + DeserializeOwned")]
    pub version: V,
    pub os: Os,
    pub arch: Arch,
    pub url: String,
    #[serde(bound = "D: Digest")]
    pub checksum: Checksum<D>,
}

impl<V: Display, D> Display for Artifact<V, D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}-{})", self.version, self.os, self.arch)
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Os {
    Darwin,
    Linux,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Arch {
    Amd64,
    Arm64,
}

impl Display for Os {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Os::Darwin => write!(f, "darwin"),
            Os::Linux => write!(f, "linux"),
        }
    }
}

#[derive(thiserror::Error, Debug)]
#[error("OS is not supported: {0}")]
pub struct UnsupportedOsError(String);

impl FromStr for Os {
    type Err = UnsupportedOsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "linux" => Ok(Os::Linux),
            "darwin" | "osx" => Ok(Os::Darwin),
            _ => Err(UnsupportedOsError(s.to_string())),
        }
    }
}

impl Display for Arch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Arch::Amd64 => write!(f, "amd64"),
            Arch::Arm64 => write!(f, "arm64"),
        }
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Arch is not supported: {0}")]
pub struct UnsupportedArchError(String);

impl FromStr for Arch {
    type Err = UnsupportedArchError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "amd64" | "x86_64" => Ok(Arch::Amd64),
            "arm64" | "aarch64" => Ok(Arch::Arm64),
            _ => Err(UnsupportedArchError(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::version::VersionRequirement;

    #[test]
    fn test_arch_display_format() {
        let archs = [(Arch::Amd64, "amd64"), (Arch::Arm64, "arm64")];

        for (input, expected) in archs {
            assert_eq!(expected, input.to_string());
        }
    }

    #[test]
    fn test_arch_parsing() {
        let archs = [
            ("amd64", Arch::Amd64),
            ("arm64", Arch::Arm64),
            ("x86_64", Arch::Amd64),
            ("aarch64", Arch::Arm64),
        ];
        for (input, expected) in archs {
            assert_eq!(expected, input.parse::<Arch>().unwrap());
        }

        assert!(matches!(
            "foo".parse::<Arch>().unwrap_err(),
            UnsupportedArchError(..)
        ));
    }

    #[test]
    fn test_os_display_format() {
        assert_eq!("linux", Os::Linux.to_string());
    }

    #[test]
    fn test_os_parsing() {
        assert_eq!(Os::Linux, "linux".parse::<Os>().unwrap());
        assert_eq!(Os::Darwin, "darwin".parse::<Os>().unwrap());
        assert_eq!(Os::Darwin, "osx".parse::<Os>().unwrap());

        assert!(matches!(
            "foo".parse::<Os>().unwrap_err(),
            UnsupportedOsError(..)
        ));
    }

    /*
    #[test]
    fn test_artifact_display() {
        assert_eq!(
            "foo (linux-arm64)",
            Artifact {
                version: String::from("foo"),
                os: Os::Linux,
                arch: Arch::Arm64,
                url: "https://example.com/".to_string(),
                checksum: Checksum::<sha2::Sha256>::try_from(String::from(
                    "7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069"
                ))
                .unwrap(),
            }
            .to_string()
        );
    }*/

    impl VersionRequirement<String> for String {
        fn satisfies(&self, version: &String) -> bool {
            self == version
        }
    }
}
