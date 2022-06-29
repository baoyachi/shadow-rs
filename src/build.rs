use std::collections::BTreeMap;

pub type ShadowConst = &'static str;

pub trait ShadowGen {
    fn gen_const(&self) -> BTreeMap<ShadowConst, ConstVal>;
}

#[derive(Debug, Clone)]
pub struct ConstVal {
    pub desc: String,
    pub v: String,
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

#[derive(Debug, Clone)]
pub enum ConstType {
    OptStr,
    Str,
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
