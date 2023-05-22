/// [`CiType`] holds the types of CI environment that `shadow-rs` can detect.
#[derive(Debug)]
pub enum CiType {
    Github,
    Gitlab,
    // TODO: Recognize other CI types, especially Travis and Jenkins
    None,
}

impl Default for CiType {
    fn default() -> Self {
        Self::None
    }
}

impl ToString for CiType {
    fn to_string(&self) -> String {
        match self {
            CiType::Github => "github".into(),
            CiType::Gitlab => "gitlab".into(),
            _ => "none".into(),
        }
    }
}
