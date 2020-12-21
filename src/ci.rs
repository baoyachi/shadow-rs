#[derive(Debug)]
pub enum CIType {
    Github,
    Gitlab,
    // Jenkins,
    // Travis,
    None,
}

impl Default for CIType {
    fn default() -> Self {
        Self::None
    }
}

impl ToString for CIType {
    fn to_string(&self) -> String {
        match self {
            CIType::Github => "github".into(),
            CIType::Gitlab => "gitlab".into(),
            // CIType::Jenkins => "jenkins".into(),
            // CIType::Travis => "travis".into(),
            _ => "none".into(),
        }
    }
}
