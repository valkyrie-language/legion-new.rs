use super::*;


impl Display for ProjectType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Workspace => write!(f, "📚 Workspace"),
            Self::Package => write!(f, "📗 Package"),
        }
    }
}
