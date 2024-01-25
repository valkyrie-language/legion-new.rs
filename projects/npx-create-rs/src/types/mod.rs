use crate::NpxError;
use dialoguer::{Select, theme::ColorfulTheme};
use std::fmt::{Display, Formatter};
use clap::ValueEnum;

mod package_type;
mod project_type;

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum ProjectType {
    Workspace,
    Package,
}
#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum PackageType {
    Library,
    Commands,
}
