use clap::{arg, Parser};
use inventory::artifact::{Arch, Artifact, Os};
use inventory::checksum::Checksum;
use inventory::inventory::Inventory;
use std::path::PathBuf;

#[derive(clap::Parser, Debug)]
#[clap(disable_version_flag = true)]
#[command(version, about, long_about = None)]
enum Args {
    /// Adds an artifact to an existing inventory file
    Add(AddSubcommand),
}

#[derive(clap::Parser, Debug)]
struct AddSubcommand {
    #[arg(index = 1)]
    path: PathBuf,
    #[arg(short, long)]
    version: String,
    #[arg(short, long)]
    os: Os,
    #[arg(short, long)]
    arch: Arch,
    #[arg(short, long)]
    url: String,
    #[arg(short, long)]
    checksum: Checksum<()>,
}

fn main() {
    let args = Args::parse();

    match args {
        Args::Add(args) => {
            let mut inventory = std::fs::read_to_string(&args.path)
                .unwrap()
                .parse::<Inventory<String, ()>>()
                .unwrap();

            inventory.push(Artifact {
                version: args.version,
                os: args.os,
                arch: args.arch,
                url: args.url,
                checksum: args.checksum,
            });

            std::fs::write(&args.path, inventory.to_string()).unwrap();
        }
    }
}
