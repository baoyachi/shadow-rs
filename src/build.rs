use std::fmt::{Display, Formatter};

/// `shadow-rs` build constant identifiers.
pub type ShadowConst = &'static str;

/// Serialized values for build constants.
#[derive(Debug, Clone)]
pub struct ConstVal {
    /// User-facing documentation for the build constant.
    pub desc: String,
    /// Serialized value of the build constant.
    pub v: String,
    /// Type of the build constant.
    pub t: ConstType,
}

impl ConstVal {
    pub fn new<S: Into<String>>(desc: S) -> ConstVal {
        ConstVal {
            desc: desc.into(),
            v: "".to_string(),
            t: ConstType::OptStr,
        }
    }

    pub fn new_bool<S: Into<String>>(desc: S) -> ConstVal {
        ConstVal {
            desc: desc.into(),
            v: "true".to_string(),
            t: ConstType::Bool,
        }
    }
}

/// Supported types of build constants.
#[derive(Debug, Clone)]
pub enum ConstType {
    /// [`Option<&str>`](`Option`).
    OptStr,
    /// [`&str`](`str`).
    Str,
    /// [`bool`].
    Bool,
}

impl Display for ConstType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstType::OptStr => write!(f, "Option<&str>"),
            ConstType::Str => write!(f, "&str"),
            ConstType::Bool => write!(f, "bool"),
        }
    }
}
