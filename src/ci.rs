#[derive(Debug)]
pub enum CiType {
    Github,
    Gitlab,
    // Jenkins,
    // Travis,
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
            // CIType::Jenkins => "jenkins".into(),
            // CIType::Travis => "travis".into(),
            _ => "none".into(),
        }
    }
}
