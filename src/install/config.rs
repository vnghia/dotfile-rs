use std::path::Path;

use clap::ValueEnum;
use const_format::formatc;

use super::binary::{ArchiveType, Binary};
use crate::constant::target::TARGET_TRIPLET;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum InstallConfig {
    Starship,
}

pub const STARSHIP_BINARY: Binary<[&str; 1]> = Binary {
    name: "starship",
    url: formatc!(
        "https://github.com/starship/starship/releases/latest/download/starship-{}.tar.gz",
        TARGET_TRIPLET
    ),
    archive: Some((ArchiveType::TarGz, Some(["starship"]))),
    version_arg: "--version",
    phantom_c: std::marker::PhantomData,
    phantom_t: std::marker::PhantomData,
};

impl InstallConfig {
    pub fn download<PB: AsRef<Path>>(self, bin_dir: PB) {
        match self {
            InstallConfig::Starship => STARSHIP_BINARY.download(bin_dir),
        }
    }
}