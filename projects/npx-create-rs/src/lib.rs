pub mod commands;
mod errors;
pub use crate::errors::NpxError;
use clap::Parser;

mod types;

pub use crate::types::{PackageType, ProjectType};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct LegionCLI {
    /// The project directory
    target: Option<String>,
    #[arg(long)]
    name: Option<String>,
    #[arg(long)]
    project_type: Option<ProjectType>,
    #[arg(long)]
    package_type: Option<PackageType>,
}
