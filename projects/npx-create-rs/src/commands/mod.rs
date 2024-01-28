use crate::{JspmType, LegionCLI, NpxError, PackageType, types::ProjectType};
use dialoguer::{Input, Select, theme::ColorfulTheme};

impl LegionCLI {
    pub async fn run(self) -> Result<(), NpxError> {
        let _name = self.ask_name()?;
        match self.ask_project_type()? {
            ProjectType::Workspace => {}
            ProjectType::Package => {
                let _package = PackageType::ask()?;
            }
        }
        let _jspm = JspmType::ask()?;
        Ok(())
    }
    fn ask_name(&self) -> Result<String, NpxError> {
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
    pub fn ask_project_type(&self) -> Result<ProjectType, NpxError> {
        match self.project_type {
            Some(s) => Ok(s),
            None => {
                let choices = &[ProjectType::Workspace, ProjectType::Package];
                let selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Do you want to make a workspace?")
                    .default(0)
                    .items(choices)
                    .interact()?;
                Ok(choices[selection])
            }
        }
    }
}
