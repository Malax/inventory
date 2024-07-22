# Inventory

## What

Decouple your downloads with the inventory crate. [Cloud Native Buildpacks (CNB)s](https://github.com/heroku/buildpacks) need to download pre-built binaries, for example, the [JVM buildpack](https://github.com/heroku/buildpacks-jvm) downloads the JVM. So far, so good.

But how does the buildpack know where to download a pre-built binary? Who builds the binary? What about compliance and safety? That's where the inventory crate comes in.

When a binary is created: the version, OS, CPU architecture, checksum, URL, and other relevant information is stored in file in TOML format. These are called manifest files. The buildpack uses the inventory crate to read the manifest file and turn that data into a query-able store

## Install

Add this line to your `Cargo.toml`:

```toml
inventory = { github = "https://github.com/Malax/inventory" }
```

Run:

```shell
$ cargo build
```

## Use in a buildpack

In a buildpack you can Read from a manifest file and use that information to find the right URL to download:

```no_run,rust
use inventory::{artifact::Arch, artifact::Os, inventory::Inventory};
use semver::{Version, VersionReq};

#[cfg(feature = "sha2")]
use sha2::Sha512;

#[cfg(feature = "sha2")]
#[cfg(feature = "semver")]
fn inventory() {
    let inventory: Inventory<Version, Sha512, Option<()>> =
    std::fs::read_to_string("inventory.toml")
        .unwrap()
        .parse()
        .unwrap();

    let requirement = VersionReq::parse("= 1.0.0").unwrap();

    if let Some(suggested) = inventory.partial_resolve(Os::Linux, Arch::Amd64, &requirement) {
        if let Some(artifact) = inventory.resolve(Os::Linux, Arch::Amd64, &requirement) {
            if suggested != artifact {
                println!(
                    "A more recent version is detected: {}, we recommend you upgrade",
                    suggested.version
                )
            }
            println!("Installing {requirement:?} from {}", artifact.url);
        } else {
            panic!("Could not install artifact {requirement:?} from inventory.toml");
        }
    } else {
        println!("Could not resolve artifact {requirement:?} from inventory.toml");
    };
}
```

## Use to build a manifest file

```shell
$ echo "TODO"
```

## Features

- `semver` - Implements the traits required to use this crate with `semver` crate (such as `semver::Version`).
- `sha2` - Implements the traits required to use this crate with the `sha2` crate (such as `sha2::512`).
