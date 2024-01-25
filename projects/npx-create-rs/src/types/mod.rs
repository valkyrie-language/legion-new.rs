use crate::NpxError;
use dialoguer::{Select, theme::ColorfulTheme};
use std::fmt::{Display, Formatter};

mod package_type;
mod project_type;

#[derive(Copy, Clone, Debug)]
pub enum ProjectType {
    Workspace,
    Package,
}
#[derive(Copy, Clone, Debug)]
pub enum PackageType {
    Library,
    Commands,
}
