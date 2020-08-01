use chrono::Local;

#[derive(Default, Debug)]
struct Shadow {
    project: Project
}

#[derive(Default, Debug)]
struct Environment {
    os: String,
    rust_version: String,
    rust_channel: String,
    cargo_version: String,
    cargo_tree: String,
    cargo_lock: String,
}

#[derive(Debug)]
enum BuildChannel {
    Debug,
    Release,
}

#[derive(Default, Debug)]
struct Project {
    pkg_name: String,
    build_time: String,
    release_channel: Option<BuildChannel>,
}

impl Project {
    fn get_project(&mut self) {
        self.pkg_name = env!("CARGO_PKG_NAME").into();
        self.build_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    }
}


impl Shadow {}

struct Git {
    tag: String,
    branch: String,
    commit_hash: String,
    short_commit_hash: String,
    git_version: String,
    commit_date: String,
    contributor: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project() {
        let mut project = Project::default();
        project.get_project();
        println!("{:?}",project);
    }
}