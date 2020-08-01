#[derive(Debug)]
pub enum BuildChannel {
    Debug,
    Release,
}

impl Default for BuildChannel {
    fn default() -> Self {
        BuildChannel::Debug
    }
}


pub fn build_channel() -> BuildChannel {
    if cfg!(debug_assertions) {
        return BuildChannel::Debug;
    }
    return BuildChannel::Release;
}