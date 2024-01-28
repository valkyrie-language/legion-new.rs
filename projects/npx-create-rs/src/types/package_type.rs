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

}
