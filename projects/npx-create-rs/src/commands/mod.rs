use crate::{LegionCLI, NpxError, PackageType, ProjectType, errors::NpxErrorKind};
use dialoguer::{Input, Select, theme::ColorfulTheme};
use std::fmt::{Display, Formatter};

impl LegionCLI {
    pub async fn run(self) -> Result<(), NpxError> {
        let name = self.get_name()?;
        match ProjectType::ask()? {
            ProjectType::Workspace => {}
            ProjectType::Package => {
                let package = PackageType::ask()?;
            }
        }
        Ok(())
    }
    fn get_name(&self) -> Result<String, NpxError> {
        match self.name.as_ref() {
            Some(s) => Ok(s.to_string()),
            None => {
                let text = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter the name of the executable:")
                    .interact_text()?;
                Ok(text)
            }
        }
    }
}
