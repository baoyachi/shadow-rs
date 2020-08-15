#[derive(Default, Debug)]
pub struct ConstMessage {
    pub desc: String,
    pub key: String,
    pub val: String,
    pub t: ConstType,
}

#[derive(Debug)]
pub enum ConstType {
    OptStr,
    Str,
}

impl Default for ConstType {
    fn default() -> Self {
        ConstType::OptStr
    }
}

impl ToString for ConstType {
    fn to_string(&self) -> String {
        match self {
            ConstType::OptStr => "Option<&str>".to_string(),
            ConstType::Str => "&str".to_string(),
        }
    }
}
