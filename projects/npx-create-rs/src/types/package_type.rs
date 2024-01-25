use super::*;



impl Display for PackageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Library => write!(f, "🎓 Library"),
            Self::Commands => write!(f, "📟 Commands"),
        }
    }
}

impl PackageType {
    pub fn ask() -> Result<PackageType, NpxError> {
        let choices = &[PackageType::Library, PackageType::Commands];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What kind of project is this?")
            .default(0)
            .items(choices)
            .interact()?;
        Ok(choices[selection])
    }
}
