mod binary;

use std::path::PathBuf;

use clap::{Args, CommandFactory};

use self::binary::{ArchiveType, Binary};
use super::constant::BINDIR_KEY;
use crate::Cli;

#[derive(Debug, Args)]
pub struct InstallArgs {
    /// Directory to install the binary into, default to `$BINDIR`.
    #[arg(short, long)]
    pub bin_dir: Option<PathBuf>,
    #[command(flatten)]
    pub binary: BinaryArgs,
}

#[derive(Debug, Args)]
pub struct BinaryArgs {
    /// Name of the binary.
    #[arg(short, long)]
    pub name: String,
    /// Url to download binary.
    #[arg(short, long)]
    pub url: String,
    /// Archive type of the url
    #[arg(short = 't', long, value_enum)]
    pub archive_type: Option<ArchiveType>,
    /// The path to the binary inside archive.
    #[arg(short = 'p', long)]
    pub archive_paths: Option<Vec<String>>,
    /// Arg to print the version info of the downloaded binary.
    /// A `^` can be addded to the beginning to avoid parsing error.
    #[arg(short = 'a', long)]
    pub version_arg: String,
}

pub fn entry_install(args: InstallArgs) {
    let bin_dir = args.bin_dir.unwrap_or_else(|| {
        if let Ok(bin_dir) = std::env::var(BINDIR_KEY) {
            bin_dir.into()
        } else {
            Cli::command()
                .error(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "--bin-dir is required if environment `BINDIR` is empty",
                )
                .exit()
        }
    });
    Binary::from(&args.binary).download(bin_dir);
}