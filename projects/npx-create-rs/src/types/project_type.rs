use super::*;

impl Display for ProjectType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Workspace => write!(f, "📚 Workspace"),
            Self::Package => write!(f, "📗 Package"),
        }
    }
}

impl ProjectType {
    pub fn ask() -> Result<ProjectType, NpxError> {
        let choices = &[ProjectType::Workspace, ProjectType::Package];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to make a workspace?")
            .default(0)
            .items(choices)
            .interact()?;
        Ok(choices[selection])
    }
}
