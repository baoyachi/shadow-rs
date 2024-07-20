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
            t: ConstType::Str,
        }
    }

    pub fn new_bool<S: Into<String>>(desc: S) -> ConstVal {
        ConstVal {
            desc: desc.into(),
            v: "true".to_string(),
            t: ConstType::Bool,
        }
    }

    pub fn new_slice<S: Into<String>>(desc: S) -> ConstVal {
        ConstVal {
            desc: desc.into(),
            v: "".to_string(),
            t: ConstType::Slice,
        }
    }
}

/// Supported types of build constants.
#[derive(Debug, Clone)]
pub enum ConstType {
    /// [`&str`](`str`).
    Str,
    /// [`bool`].
    Bool,
    /// [`&[u8]`].
    Slice,
}

impl Display for ConstType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstType::Str => write!(f, "&str"),
            ConstType::Bool => write!(f, "bool"),
            ConstType::Slice => write!(f, "&[u8]"),
        }
    }
}
