#[derive(Debug)]
pub enum BuildRustChannel {
    Debug,
    Release,
}

impl Default for BuildRustChannel {
    fn default() -> Self {
        BuildRustChannel::Debug
    }
}

pub fn build_channel() -> BuildRustChannel {
    if cfg!(debug_assertions) {
        return BuildRustChannel::Debug;
    }
    return BuildRustChannel::Release;
}

impl ToString for BuildRustChannel {
    fn to_string(&self) -> String {
        match self {
            BuildRustChannel::Debug => "debug".to_string(),
            BuildRustChannel::Release => "release".to_string(),
        }
    }
}
