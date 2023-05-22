use std::collections::BTreeMap;

/// `shadow-rs` build constant identifiers.
pub type ShadowConst = &'static str;

pub trait ShadowGen {
    fn gen_const(&self) -> BTreeMap<ShadowConst, ConstVal>;
}

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

impl ToString for ConstType {
    fn to_string(&self) -> String {
        match self {
            ConstType::OptStr => "Option<&str>".to_string(),
            ConstType::Str => "&str".to_string(),
            ConstType::Bool => "bool".to_string(),
        }
    }
}
