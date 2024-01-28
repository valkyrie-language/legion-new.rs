use crate::NpxError;
use clap::ValueEnum;
use dialoguer::{Select, theme::ColorfulTheme};
use std::fmt::{Display, Formatter};

mod jspm_type;
mod package_type;
mod project_type;

#[derive(Copy, Clone, Debug, Default, ValueEnum)]
pub enum ProjectType {
    #[default]
    Workspace,
    Package,
}

#[derive(Copy, Clone, Debug, Default, ValueEnum)]
pub enum PackageType {
    #[default]
    Library,
    Commands,
}

#[derive(Copy, Clone, Debug, Default, ValueEnum)]
pub enum JspmType {
    #[default]
    Npm,
    Pnpm,
    Yarn,
}
