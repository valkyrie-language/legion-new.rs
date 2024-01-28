pub mod commands;
mod errors;
pub use crate::errors::NpxError;
use clap::Parser;

mod types;

use crate::types::ProjectType;
pub use crate::types::{JspmType, PackageType};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct LegionNew {
    /// The project directory
    target: Option<String>,
    /// The project name
    #[arg(long)]
    name: Option<String>,
    /// The project type
    #[arg(long)]
    project_type: Option<ProjectType>,
    /// The package type
    #[arg(long)]
    package_type: Option<PackageType>,
    /// The javascript package manager
    #[arg(long, visible_alias = "jspm")]
    javascript_package_manager: Option<JspmType>,
    /// Dry run
    #[arg(long)]
    dry_run: bool,
}
