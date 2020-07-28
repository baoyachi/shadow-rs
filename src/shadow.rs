struct Shadow {}

struct Environment {
    os: String,
    rust_version: String,
    rust_channel: String,
    cargo_version: String,
    cargo_tree: String,
    cargo_lock: String,
}

enum BuildChannel {
    Debug,
    Release,
}

struct Project {
    project_version: String,
    build_time: String,
    release_channel: BuildChannel,
}

struct Git {
    tag: String,
    branch: String,
    commit_hash: String,
    short_commit_hash: String,
    git_version: String,
    commit_date: String,
    contributor: String,
}