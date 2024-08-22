use crate::build::{ConstType, ConstVal, ShadowConst};
use crate::ci::CiType;
use crate::err::*;
use crate::{DateTime, Format};
use std::collections::BTreeMap;
use std::io::{BufReader, Read};
use std::path::Path;
use std::process::{Command, Stdio};

const BRANCH_DOC: &str = r#"
The name of the Git branch that this project was built from.
This constant will be empty if the branch cannot be determined."#;
pub const BRANCH: ShadowConst = "BRANCH";
const TAG_DOC: &str = r#"
The name of the Git tag that this project was built from.
Note that this will be empty if there is no tag for the HEAD at the time of build."#;
pub const TAG: ShadowConst = "TAG";
const LAST_TAG_DOC: &str = r#"
The name of the last Git tag on the branch that this project was built from.
As opposed to [`TAG`], this does not require the current commit to be tagged, just one of its parents.

This constant will be empty if the last tag cannot be determined."#;
pub const LAST_TAG: ShadowConst = "LAST_TAG";
const SHORT_COMMIT_DOC: &str = r#"
The short hash of the Git commit that this project was built from.
Note that this will always truncate [`COMMIT_HASH`] to 8 characters if necessary.
Depending on the amount of commits in your project, this may not yield a unique Git identifier
([see here for more details on hash abbreviation](https://git-scm.com/docs/git-describe#_examples)).

This constant will be empty if the last commit cannot be determined."#;
pub const SHORT_COMMIT: ShadowConst = "SHORT_COMMIT";
const COMMIT_HASH_DOC: &str = r#"
The full commit hash of the Git commit that this project was built from.
An abbreviated, but not necessarily unique, version of this is [`SHORT_COMMIT`].

This constant will be empty if the last commit cannot be determined."#;
pub const COMMIT_HASH: ShadowConst = "COMMIT_HASH";
const COMMIT_DATE_DOC: &str = r#"The time of the Git commit that this project was built from.
The time is formatted in modified ISO 8601 format (`YYYY-MM-DD HH-MM ±hh-mm` where hh-mm is the offset from UTC).

This constant will be empty if the last commit cannot be determined."#;
pub const COMMIT_DATE: ShadowConst = "COMMIT_DATE";
const COMMIT_DATE_2822_DOC: &str = r#"
The name of the Git branch that this project was built from.
The time is formatted according to [RFC 2822](https://datatracker.ietf.org/doc/html/rfc2822#section-3.3) (e.g. HTTP Headers).

This constant will be empty if the last commit cannot be determined."#;
pub const COMMIT_DATE_2822: ShadowConst = "COMMIT_DATE_2822";
const COMMIT_DATE_3339_DOC: &str = r#"
The name of the Git branch that this project was built from.
The time is formatted according to [RFC 3339 and ISO 8601](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6).

This constant will be empty if the last commit cannot be determined."#;
pub const COMMIT_DATE_3339: ShadowConst = "COMMIT_DATE_3339";
const COMMIT_AUTHOR_DOC: &str = r#"
The author of the Git commit that this project was built from.

This constant will be empty if the last commit cannot be determined."#;
pub const COMMIT_AUTHOR: ShadowConst = "COMMIT_AUTHOR";
const COMMIT_EMAIL_DOC: &str = r#"
The e-mail address of the author of the Git commit that this project was built from.

This constant will be empty if the last commit cannot be determined."#;
pub const COMMIT_EMAIL: ShadowConst = "COMMIT_EMAIL";
const GIT_CLEAN_DOC: &str = r#"
Whether the Git working tree was clean at the time of project build (`true`), or not (`false`).

This constant will be `false` if the last commit cannot be determined."#;
pub const GIT_CLEAN: ShadowConst = "GIT_CLEAN";
const GIT_STATUS_FILE_DOC: &str = r#"
The Git working tree status as a list of files with their status, similar to `git status`.
Each line of the list is preceded with `  * `, followed by the file name.
Files marked `(dirty)` have unstaged changes.
Files marked `(staged)` have staged changes.

This constant will be empty if the working tree status cannot be determined."#;
pub const GIT_STATUS_FILE: ShadowConst = "GIT_STATUS_FILE";

#[derive(Default, Debug)]
pub struct Git {
    map: BTreeMap<ShadowConst, ConstVal>,
    ci_type: CiType,
}

impl Git {
    fn update_str(&mut self, c: ShadowConst, v: String) {
        if let Some(val) = self.map.get_mut(c) {
            *val = ConstVal {
                desc: val.desc.clone(),
                v,
                t: ConstType::Str,
            }
        }
    }

    fn update_bool(&mut self, c: ShadowConst, v: bool) {
        if let Some(val) = self.map.get_mut(c) {
            *val = ConstVal {
                desc: val.desc.clone(),
                v: v.to_string(),
                t: ConstType::Bool,
            }
        }
    }

    fn init(&mut self, path: &Path, std_env: &BTreeMap<String, String>) -> SdResult<()> {
        // First, try executing using the git command.
        if let Err(err) = self.init_git() {
            println!("{err}");
        }

        // If the git2 feature is enabled, then replace the corresponding values with git2.
        self.init_git2(path)?;

        // use command branch
        if let Some(x) = find_branch_in(path) {
            self.update_str(BRANCH, x)
        };

        // use command tag
        if let Some(x) = command_current_tag() {
            self.update_str(TAG, x)
        }

        // use command get last tag
        if let Some(x) = command_last_tag() {
            self.update_str(LAST_TAG, x)
        }

        // try use ci branch,tag
        self.ci_branch_tag(std_env);
        Ok(())
    }

    fn init_git(&mut self) -> SdResult<()> {
        // check git status
        let x = command_git_clean();
        self.update_bool(GIT_CLEAN, x);

        let x = command_git_status_file();
        self.update_str(GIT_STATUS_FILE, x);

        let git_info = command_git_head();

        self.update_str(COMMIT_EMAIL, git_info.email);
        self.update_str(COMMIT_AUTHOR, git_info.author);
        self.update_str(SHORT_COMMIT, git_info.short_commit);
        self.update_str(COMMIT_HASH, git_info.commit);

        let time_stamp = git_info.date.parse::<i64>()?;
        if let Ok(date_time) = DateTime::timestamp_2_utc(time_stamp) {
            self.update_str(COMMIT_DATE, date_time.human_format());
            self.update_str(COMMIT_DATE_2822, date_time.to_rfc2822());
            self.update_str(COMMIT_DATE_3339, date_time.to_rfc3339());
        }

        Ok(())
    }

    #[allow(unused_variables)]
    fn init_git2(&mut self, path: &Path) -> SdResult<()> {
        #[cfg(feature = "git2")]
        {
            use crate::date_time::DateTime;
            use crate::git::git2_mod::git_repo;
            use crate::Format;

            let repo = git_repo(path).map_err(ShadowError::new)?;
            let reference = repo.head().map_err(ShadowError::new)?;

            //get branch
            let branch = reference
                .shorthand()
                .map(|x| x.trim().to_string())
                .or_else(command_current_branch)
                .unwrap_or_default();

            //get HEAD branch
            let tag = command_current_tag().unwrap_or_default();
            let last_tag = command_last_tag().unwrap_or_default();
            self.update_str(BRANCH, branch);
            self.update_str(TAG, tag);
            self.update_str(LAST_TAG, last_tag);

            if let Some(v) = reference.target() {
                let commit = v.to_string();
                self.update_str(COMMIT_HASH, commit.clone());
                let mut short_commit = commit.as_str();

                if commit.len() > 8 {
                    short_commit = short_commit.get(0..8).unwrap();
                }
                self.update_str(SHORT_COMMIT, short_commit.to_string());
            }

            let commit = reference.peel_to_commit().map_err(ShadowError::new)?;

            let author = commit.author();
            if let Some(v) = author.email() {
                self.update_str(COMMIT_EMAIL, v.to_string());
            }

            if let Some(v) = author.name() {
                self.update_str(COMMIT_AUTHOR, v.to_string());
            }
            let status_file = Self::git2_dirty_stage(&repo);
            if status_file.trim().is_empty() {
                self.update_bool(GIT_CLEAN, true);
            } else {
                self.update_bool(GIT_CLEAN, false);
            }
            self.update_str(GIT_STATUS_FILE, status_file);

            let time_stamp = commit.time().seconds().to_string().parse::<i64>()?;
            if let Ok(date_time) = DateTime::timestamp_2_utc(time_stamp) {
                self.update_str(COMMIT_DATE, date_time.human_format());

                self.update_str(COMMIT_DATE_2822, date_time.to_rfc2822());

                self.update_str(COMMIT_DATE_3339, date_time.to_rfc3339());
            }
        }
        Ok(())
    }

    //use git2 crates git repository 'dirty or stage' status files.
    #[cfg(feature = "git2")]
    pub fn git2_dirty_stage(repo: &git2::Repository) -> String {
        let mut repo_opts = git2::StatusOptions::new();
        repo_opts.include_ignored(false);
        if let Ok(statue) = repo.statuses(Some(&mut repo_opts)) {
            let mut dirty_files = Vec::new();
            let mut staged_files = Vec::new();

            for status in statue.iter() {
                if let Some(path) = status.path() {
                    match status.status() {
                        git2::Status::CURRENT => (),
                        git2::Status::INDEX_NEW
                        | git2::Status::INDEX_MODIFIED
                        | git2::Status::INDEX_DELETED
                        | git2::Status::INDEX_RENAMED
                        | git2::Status::INDEX_TYPECHANGE => staged_files.push(path.to_string()),
                        _ => dirty_files.push(path.to_string()),
                    };
                }
            }
            filter_git_dirty_stage(dirty_files, staged_files)
        } else {
            "".into()
        }
    }

    #[allow(clippy::manual_strip)]
    fn ci_branch_tag(&mut self, std_env: &BTreeMap<String, String>) {
        let mut branch: Option<String> = None;
        let mut tag: Option<String> = None;
        match self.ci_type {
            CiType::Gitlab => {
                if let Some(v) = std_env.get("CI_COMMIT_TAG") {
                    tag = Some(v.to_string());
                } else if let Some(v) = std_env.get("CI_COMMIT_REF_NAME") {
                    branch = Some(v.to_string());
                }
            }
            CiType::Github => {
                if let Some(v) = std_env.get("GITHUB_REF") {
                    let ref_branch_prefix: &str = "refs/heads/";
                    let ref_tag_prefix: &str = "refs/tags/";

                    if v.starts_with(ref_branch_prefix) {
                        branch = Some(
                            v.get(ref_branch_prefix.len()..)
                                .unwrap_or_default()
                                .to_string(),
                        )
                    } else if v.starts_with(ref_tag_prefix) {
                        tag = Some(
                            v.get(ref_tag_prefix.len()..)
                                .unwrap_or_default()
                                .to_string(),
                        )
                    }
                }
            }
            _ => {}
        }
        if let Some(x) = branch {
            self.update_str(BRANCH, x);
        }

        if let Some(x) = tag {
            self.update_str(TAG, x.clone());
            self.update_str(LAST_TAG, x);
        }
    }
}

pub(crate) fn new_git(
    path: &Path,
    ci: CiType,
    std_env: &BTreeMap<String, String>,
) -> BTreeMap<ShadowConst, ConstVal> {
    let mut git = Git {
        map: Default::default(),
        ci_type: ci,
    };
    git.map.insert(BRANCH, ConstVal::new(BRANCH_DOC));

    git.map.insert(TAG, ConstVal::new(TAG_DOC));

    git.map.insert(LAST_TAG, ConstVal::new(LAST_TAG_DOC));

    git.map.insert(COMMIT_HASH, ConstVal::new(COMMIT_HASH_DOC));

    git.map
        .insert(SHORT_COMMIT, ConstVal::new(SHORT_COMMIT_DOC));

    git.map
        .insert(COMMIT_AUTHOR, ConstVal::new(COMMIT_AUTHOR_DOC));
    git.map
        .insert(COMMIT_EMAIL, ConstVal::new(COMMIT_EMAIL_DOC));
    git.map.insert(COMMIT_DATE, ConstVal::new(COMMIT_DATE_DOC));

    git.map
        .insert(COMMIT_DATE_2822, ConstVal::new(COMMIT_DATE_2822_DOC));

    git.map
        .insert(COMMIT_DATE_3339, ConstVal::new(COMMIT_DATE_3339_DOC));

    git.map.insert(GIT_CLEAN, ConstVal::new_bool(GIT_CLEAN_DOC));

    git.map
        .insert(GIT_STATUS_FILE, ConstVal::new(GIT_STATUS_FILE_DOC));

    if let Err(e) = git.init(path, std_env) {
        println!("{e}");
    }

    git.map
}

#[cfg(feature = "git2")]
pub mod git2_mod {
    use git2::Error as git2Error;
    use git2::Repository;
    use std::path::Path;

    pub fn git_repo<P: AsRef<Path>>(path: P) -> Result<Repository, git2Error> {
        Repository::discover(path)
    }

    pub fn git2_current_branch(repo: &Repository) -> Option<String> {
        repo.head()
            .map(|x| x.shorthand().map(|x| x.to_string()))
            .unwrap_or(None)
    }
}

/// get current repository git branch.
///
/// When current repository exists git folder.
///
/// It's use default feature.This function try use [git2] crates get current branch.
/// If not use git2 feature,then try use [Command] to get.
pub fn branch() -> String {
    #[cfg(feature = "git2")]
    {
        use crate::git::git2_mod::{git2_current_branch, git_repo};
        git_repo(".")
            .map(|x| git2_current_branch(&x))
            .unwrap_or_else(|_| command_current_branch())
            .unwrap_or_default()
    }
    #[cfg(not(feature = "git2"))]
    {
        command_current_branch().unwrap_or_default()
    }
}

/// get current repository git tag.
///
/// When current repository exists git folder.
/// I's use [Command] to get.
pub fn tag() -> String {
    command_current_tag().unwrap_or_default()
}

/// Check current git Repository status without nothing(dirty or stage)
///
/// if nothing,It means clean:true. On the contrary, it is 'dirty':false
pub fn git_clean() -> bool {
    #[cfg(feature = "git2")]
    {
        use crate::git::git2_mod::git_repo;
        git_repo(".")
            .map(|x| Git::git2_dirty_stage(&x))
            .map(|x| x.trim().is_empty())
            .unwrap_or(true)
    }
    #[cfg(not(feature = "git2"))]
    {
        command_git_clean()
    }
}

/// List current git Repository statue(dirty or stage) contain file changed
///
/// Refer to the 'cargo fix' result output when git statue(dirty or stage) changed.
///
/// Example output:`   * examples/builtin_fn.rs (dirty)`
pub fn git_status_file() -> String {
    #[cfg(feature = "git2")]
    {
        use crate::git::git2_mod::git_repo;
        git_repo(".")
            .map(|x| Git::git2_dirty_stage(&x))
            .unwrap_or_default()
    }
    #[cfg(not(feature = "git2"))]
    {
        command_git_status_file()
    }
}

struct GitHeadInfo {
    commit: String,
    short_commit: String,
    email: String,
    author: String,
    date: String,
}

struct GitCommandExecutor<'a> {
    path: &'a Path,
}

impl Default for GitCommandExecutor<'_> {
    fn default() -> Self {
        Self::new(Path::new("."))
    }
}

impl<'a> GitCommandExecutor<'a> {
    fn new(path: &'a Path) -> Self {
        GitCommandExecutor { path }
    }

    fn exec(&self, args: &[&str]) -> Option<String> {
        Command::new("git")
            .current_dir(self.path)
            .args(args)
            .output()
            .map(|x| {
                String::from_utf8(x.stdout)
                    .map(|x| x.trim().to_string())
                    .ok()
            })
            .unwrap_or(None)
    }
}

fn command_git_head() -> GitHeadInfo {
    let cli = |args: &[&str]| GitCommandExecutor::default().exec(args).unwrap_or_default();
    GitHeadInfo {
        commit: cli(&["rev-parse", "HEAD"]),
        short_commit: cli(&["rev-parse", "--short", "HEAD"]),
        author: cli(&["log", "-1", "--pretty=format:%an"]),
        email: cli(&["log", "-1", "--pretty=format:%ae"]),
        date: cli(&["show", "--pretty=format:%ct", "--date=raw", "-s"]),
    }
}

/// Command exec git current tag
fn command_current_tag() -> Option<String> {
    GitCommandExecutor::default().exec(&["tag", "-l", "--contains", "HEAD"])
}

/// git describe --tags --abbrev=0 HEAD
/// Command exec git last tag
fn command_last_tag() -> Option<String> {
    GitCommandExecutor::default().exec(&["describe", "--tags", "--abbrev=0", "HEAD"])
}

/// git clean:git status --porcelain
/// check repository git status is clean
fn command_git_clean() -> bool {
    GitCommandExecutor::default()
        .exec(&["status", "--porcelain"])
        .map(|x| x.is_empty())
        .unwrap_or(true)
}

/// check git repository 'dirty or stage' status files.
/// git dirty:git status  --porcelain | grep '^\sM.' |awk '{print $2}'
/// git stage:git status --porcelain --untracked-files=all | grep '^[A|M|D|R]'|awk '{print $2}'
fn command_git_status_file() -> String {
    let git_status_files =
        move |args: &[&str], grep: &[&str], awk: &[&str]| -> SdResult<Vec<String>> {
            let git_shell = Command::new("git")
                .args(args)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?;
            let git_out = git_shell.stdout.ok_or("Failed to exec git stdout")?;

            let grep_shell = Command::new("grep")
                .args(grep)
                .stdin(Stdio::from(git_out))
                .stdout(Stdio::piped())
                .spawn()?;
            let grep_out = grep_shell.stdout.ok_or("Failed to exec grep stdout")?;

            let mut awk_shell = Command::new("awk")
                .args(awk)
                .stdin(Stdio::from(grep_out))
                .stdout(Stdio::piped())
                .spawn()?;
            let mut awk_out = BufReader::new(
                awk_shell
                    .stdout
                    .as_mut()
                    .ok_or("Failed to exec awk stdout")?,
            );
            let mut line = String::new();
            awk_out.read_to_string(&mut line)?;
            Ok(line.lines().map(|x| x.into()).collect())
        };

    let dirty = git_status_files(&["status", "--porcelain"], &[r"^\sM."], &["{print $2}"])
        .unwrap_or_default();

    let stage = git_status_files(
        &["status", "--porcelain", "--untracked-files=all"],
        &[r#"^[A|M|D|R]"#],
        &["{print $2}"],
    )
    .unwrap_or_default();
    filter_git_dirty_stage(dirty, stage)
}

/// Command exec git current branch
fn command_current_branch() -> Option<String> {
    find_branch_in(Path::new("."))
}

fn find_branch_in(path: &Path) -> Option<String> {
    GitCommandExecutor::new(path).exec(&["symbolic-ref", "--short", "HEAD"])
}

fn filter_git_dirty_stage(dirty_files: Vec<String>, staged_files: Vec<String>) -> String {
    let mut concat_file = String::new();
    for file in dirty_files {
        concat_file.push_str("  * ");
        concat_file.push_str(&file);
        concat_file.push_str(" (dirty)\n");
    }
    for file in staged_files {
        concat_file.push_str("  * ");
        concat_file.push_str(&file);
        concat_file.push_str(" (staged)\n");
    }
    concat_file
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_std_env;

    #[test]
    fn test_git() {
        let env_map = get_std_env();
        let map = new_git(Path::new("./"), CiType::Github, &env_map);
        for (k, v) in map {
            println!("k:{},v:{:?}", k, v);
            assert!(!v.desc.is_empty());
            if !k.eq(TAG) && !k.eq(LAST_TAG) && !k.eq(BRANCH) && !k.eq(GIT_STATUS_FILE) {
                assert!(!v.v.is_empty());
                continue;
            }

            //assert github tag always exist value
            if let Some(github_ref) = env_map.get("GITHUB_REF") {
                if github_ref.starts_with("refs/tags/") && k.eq(TAG) {
                    assert!(!v.v.is_empty(), "not empty");
                } else if github_ref.starts_with("refs/heads/") && k.eq(BRANCH) {
                    assert!(!v.v.is_empty());
                }
            }
        }
    }

    #[test]
    fn test_current_branch() {
        if get_std_env().contains_key("GITHUB_REF") {
            return;
        }
        #[cfg(feature = "git2")]
        {
            use crate::git::git2_mod::{git2_current_branch, git_repo};
            let git2_branch = git_repo(".")
                .map(|x| git2_current_branch(&x))
                .unwrap_or(None);
            let command_branch = command_current_branch();
            assert!(git2_branch.is_some());
            assert!(command_branch.is_some());
            assert_eq!(command_branch, git2_branch);
        }

        assert_eq!(Some(branch()), command_current_branch());
    }

    #[test]
    fn test_command_last_tag() {
        let opt_last_tag = command_last_tag();
        assert!(opt_last_tag.is_some())
    }
}
