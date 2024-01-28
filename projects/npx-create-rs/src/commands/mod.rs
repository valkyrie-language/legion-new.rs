use crate::{JspmType, LegionNew, NpxError, PackageType, types::ProjectType};
use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use reqwest::Url;
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

impl LegionNew {
    pub async fn run(self) -> Result<(), NpxError> {
        let target = self.ask_target()?;
        if target.exists() {
            let rm = self.ask_override(&target)?;
            if rm {
                println!("Removing `{}`", target.display());
                if !self.dry_run {
                    std::fs::remove_dir_all(&target)?;
                }
            }
            else {
                println!("Stop because exists");
                return Ok(());
            }
        }
        if !self.dry_run {
            std::fs::create_dir_all(&target)?;
        }
        let _name = self.ask_name()?;
        match self.ask_project_type()? {
            ProjectType::Workspace => {}
            ProjectType::Package => {
                let _package = self.ask_package_type()?;
            }
        }
        let _package = self.ask_package_type()?;
        let jspm = JspmType::ask()?;
        jspm.install().await?;
        //     console.log(
        //         `${chalk.green("✔")} Success! Created ${chalk.cyan(
        //             name
        //         )} at ${chalk.cyan(target)}`
        //     );
        Ok(())
    }
    fn ask_target(&self) -> Result<PathBuf, NpxError> {
        match self.target.as_ref() {
            Some(s) => Ok(PathBuf::from(s)),
            None => Ok(current_dir()?),
        }
    }
    fn ask_override(&self, path: &Path) -> Result<bool, NpxError> {
        let url = Url::from_directory_path(path).unwrap();
        let over = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Do you want to remove exists {}?", url))
            .report(false)
            .interact()?;
        Ok(over)
    }

    fn ask_name(&self) -> Result<String, NpxError> {
        match self.name.as_ref() {
            Some(s) => Ok(s.to_string()),
            None => {
                let default_name = match self.target.as_ref() {
                    Some(s) => s,
                    None => "valkyrie-legion",
                };
                let text = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter the name of the project:")
                    .default(default_name.to_string())
                    .interact_text()?;
                Ok(text)
            }
        }
    }
    fn ask_project_type(&self) -> Result<ProjectType, NpxError> {
        let choices = &[ProjectType::Workspace, ProjectType::Package];
        match self.project_type {
            Some(s) => Ok(s),
            None => {
                let selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("What kind of project is this?")
                    .default(0)
                    .items(choices)
                    .interact()?;
                Ok(choices[selection])
            }
        }
    }

    fn ask_package_type(&self) -> Result<PackageType, NpxError> {
        let choices = &[PackageType::Library, PackageType::Commands];
        match self.package_type {
            Some(s) => Ok(s),
            None => {
                let selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("What kind of package is this?")
                    .default(0)
                    .items(choices)
                    .interact()?;
                Ok(choices[selection])
            }
        }
    }
}
