use std::collections::HashMap;
use std::cell::RefCell;

pub type ShadowConst = &'static str;

pub trait ShadowGen {
    fn gen_const(&self) -> HashMap<ShadowConst, ConstVal>;
}


#[derive(Debug, Clone)]
pub struct ConstVal {
    pub desc: String,
    pub v: String,
    pub t: ConstType,
}

impl ConstVal {
    pub fn new<S: Into<String>>(desc: S) -> RefCell<ConstVal> {
        RefCell::new(ConstVal {
            desc: desc.into(),
            v: "".to_string(),
            t: ConstType::OptStr,
        })
    }
}

#[derive(Debug, Clone)]
pub enum ConstType {
    OptStr,
    Str,
}

impl ToString for ConstType {
    fn to_string(&self) -> String {
        match self {
            ConstType::OptStr => "Option<&str>".to_string(),
            ConstType::Str => "&str".to_string(),
        }
    }
}
