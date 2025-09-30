use std::fmt::{Display, Formatter};

/// [`CiType`] holds the types of CI environment that `shadow-rs` can detect.
#[derive(Debug, Default)]
pub enum CiType {
    Github,
    Gitlab,
    // TODO: Recognize other CI types, especially Travis and Jenkins
    #[default]
    None,
}

impl Display for CiType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CiType::Github => write!(f, "github"),
            CiType::Gitlab => write!(f, "gitlab"),
            _ => write!(f, "none"),
        }
    }
}
