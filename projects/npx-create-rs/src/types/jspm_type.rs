use super::*;

impl Display for JspmType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Npm => write!(f, "💚 npm"),
            Self::Pnpm => write!(f, "💙 pnpm"),
            Self::Yarn => write!(f, "💛 yarn"),
        }
    }
}

impl JspmType {
    pub fn ask() -> Result<JspmType, NpxError> {
        let choices = &[JspmType::Npm, JspmType::Pnpm, JspmType::Yarn];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select the javascript package manager: ")
            .default(1)
            .items(choices)
            .interact()?;
        Ok(choices[selection])
    }
    pub async fn install(&self) -> Result<(), NpxError> {
        let _cmd = match self {
            JspmType::Npm => "npm i",
            JspmType::Pnpm => "pnpm i",
            JspmType::Yarn => "yarn",
        };
        Ok(())
    }
}
