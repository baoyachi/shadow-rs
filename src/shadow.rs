struct Shadow {}

struct Environment {
    os: String,
    rust_version: String,
    rust_channel: String,
    cargo_version: String,
    cargo_tree: String,
    cargo_lock: String,
}

struct Project {
    project_version: String,
    build_time: String,
}

struct Git {
    tag: String,
    branch: String,
    commit_id: String,
    git_version: String,
    commit_date: String,
    contributor: String,
}