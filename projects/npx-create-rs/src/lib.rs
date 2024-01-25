pub mod commands;
mod errors;
pub use crate::errors::NpxError;
use clap::Parser;

mod types;

pub use crate::types::{ProjectType, PackageType};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct LegionCLI {
    #[arg(long)]
    name: Option<String>,
}
