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

pub const COMMITS_SINCE_TAG_DOC: &str = r#"
The number of commits since the last Git tag on the branch that this project was built from.
This value indicates how many commits have been made after the last tag and before the current commit.

If there are no additional commits after the last tag (i.e., the current commit is exactly at a tag),
this value will be `0`.

This constant will be empty or `0` if the last tag cannot be determined or if there are no commits after it.
"#;

pub const COMMITS_SINCE_TAG: &str = "COMMITS_SINCE_TAG";

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
The timezone information from the original commit is preserved.

This constant will be empty if the last commit cannot be determined."#;
pub const COMMIT_DATE: ShadowConst = "COMMIT_DATE";

const COMMIT_DATE_2822_DOC: &str = r#"
The time of the Git commit that this project was built from.
The time is formatted according to [RFC 2822](https://datatracker.ietf.org/doc/html/rfc2822#section-3.3) (e.g. HTTP Headers).
The timezone information from the original commit is preserved.

This constant will be empty if the last commit cannot be determined."#;
pub const COMMIT_DATE_2822: ShadowConst = "COMMIT_DATE_2822";

const COMMIT_DATE_3339_DOC: &str = r#"
The time of the Git commit that this project was built from.
The time is formatted according to [RFC 3339 and ISO 8601](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6).
The timezone information from the original commit is preserved.

This constant will be empty if the last commit cannot be determined."#;
pub const COMMIT_DATE_3339: ShadowConst = "COMMIT_DATE_3339";

const COMMIT_TIMESTAMP_DOC: &str = r#"
The time of the Git commit as a Unix timestamp (seconds since Unix epoch).

This constant will be empty if the last commit cannot be determined."#;
pub const COMMIT_TIMESTAMP: ShadowConst = "COMMIT_TIMESTAMP";

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

    fn update_usize(&mut self, c: ShadowConst, v: usize) {
        if let Some(val) = self.map.get_mut(c) {
            *val = ConstVal {
                desc: val.desc.clone(),
                v: v.to_string(),
                t: ConstType::Usize,
            }
        }
    }

    fn update_int(&mut self, c: ShadowConst, v: i64) {
        if let Some(val) = self.map.get_mut(c) {
            *val = ConstVal {
                desc: val.desc.clone(),
                v: v.to_string(),
                t: ConstType::Int,
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
        let describe = command_git_describe();
        if let Some(x) = describe.0 {
            self.update_str(LAST_TAG, x)
        }

        if let Some(x) = describe.1 {
            self.update_usize(COMMITS_SINCE_TAG, x)
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

        // Try to parse ISO format with timezone first, fallback to UTC timestamp
        if !git_info.date_iso.is_empty() {
            if let Ok(date_time) = DateTime::from_iso8601_string(&git_info.date_iso) {
                self.update_str(COMMIT_DATE, date_time.human_format());
                self.update_str(COMMIT_DATE_2822, date_time.to_rfc2822());
                self.update_str(COMMIT_DATE_3339, date_time.to_rfc3339());
                self.update_int(COMMIT_TIMESTAMP, date_time.timestamp());
            } else if let Ok(time_stamp) = git_info.date.parse::<i64>() {
                if let Ok(date_time) = DateTime::timestamp_2_utc(time_stamp) {
                    self.update_str(COMMIT_DATE, date_time.human_format());
                    self.update_str(COMMIT_DATE_2822, date_time.to_rfc2822());
                    self.update_str(COMMIT_DATE_3339, date_time.to_rfc3339());
                    self.update_int(COMMIT_TIMESTAMP, date_time.timestamp());
                }
            }
        } else if let Ok(time_stamp) = git_info.date.parse::<i64>() {
            if let Ok(date_time) = DateTime::timestamp_2_utc(time_stamp) {
                self.update_str(COMMIT_DATE, date_time.human_format());
                self.update_str(COMMIT_DATE_2822, date_time.to_rfc2822());
                self.update_str(COMMIT_DATE_3339, date_time.to_rfc3339());
                self.update_int(COMMIT_TIMESTAMP, date_time.timestamp());
            }
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
            self.update_str(BRANCH, branch);
            self.update_str(TAG, tag);

            // use command get last tag
            let describe = command_git_describe();
            if let Some(x) = describe.0 {
                self.update_str(LAST_TAG, x)
            }

            if let Some(x) = describe.1 {
                self.update_usize(COMMITS_SINCE_TAG, x)
            }

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

            let commit_time = commit.time();
            let time_stamp = commit_time.seconds();
            let offset_minutes = commit_time.offset_minutes();

            // Create OffsetDateTime with the commit's timezone
            if let Ok(utc_time) = time::OffsetDateTime::from_unix_timestamp(time_stamp) {
                if let Ok(offset) = time::UtcOffset::from_whole_seconds(offset_minutes * 60) {
                    let local_time = utc_time.to_offset(offset);
                    let date_time = DateTime::Local(local_time);

                    self.update_str(COMMIT_DATE, date_time.human_format());
                    self.update_str(COMMIT_DATE_2822, date_time.to_rfc2822());
                    self.update_str(COMMIT_DATE_3339, date_time.to_rfc3339());
                } else {
                    // Fallback to UTC if offset parsing fails
                    let date_time = DateTime::Utc(utc_time);
                    self.update_str(COMMIT_DATE, date_time.human_format());
                    self.update_str(COMMIT_DATE_2822, date_time.to_rfc2822());
                    self.update_str(COMMIT_DATE_3339, date_time.to_rfc3339());
                }
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

    git.map.insert(
        COMMITS_SINCE_TAG,
        ConstVal::new_usize(COMMITS_SINCE_TAG_DOC),
    );

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

    git.map
        .insert(COMMIT_TIMESTAMP, ConstVal::new(COMMIT_TIMESTAMP_DOC));

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
    date_iso: String,
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
            .env("GIT_OPTIONAL_LOCKS", "0")
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
        date_iso: cli(&["log", "-1", "--pretty=format:%cI"]),
    }
}

/// Command exec git current tag
fn command_current_tag() -> Option<String> {
    GitCommandExecutor::default().exec(&["tag", "-l", "--contains", "HEAD"])
}

/// git describe --tags HEAD
/// Command exec git describe
fn command_git_describe() -> (Option<String>, Option<usize>, Option<String>) {
    let last_tag =
        GitCommandExecutor::default().exec(&["describe", "--tags", "--abbrev=0", "HEAD"]);
    if last_tag.is_none() {
        return (None, None, None);
    }

    let tag = last_tag.unwrap();

    let describe = GitCommandExecutor::default().exec(&["describe", "--tags", "HEAD"]);
    if let Some(desc) = describe {
        match parse_git_describe(&tag, &desc) {
            Ok((tag, commits, hash)) => {
                return (Some(tag), commits, hash);
            }
            Err(_) => {
                return (Some(tag), None, None);
            }
        }
    }
    (Some(tag), None, None)
}

fn parse_git_describe(
    last_tag: &str,
    describe: &str,
) -> SdResult<(String, Option<usize>, Option<String>)> {
    if !describe.starts_with(last_tag) {
        return Err(ShadowError::String("git describe result error".to_string()));
    }

    if last_tag == describe {
        return Ok((describe.to_string(), None, None));
    }

    let parts: Vec<&str> = describe.rsplit('-').collect();

    if parts.is_empty() || parts.len() == 2 {
        return Err(ShadowError::String(
            "git describe result error,expect:<tag>-<num_commits>-g<hash>".to_string(),
        ));
    }

    if parts.len() > 2 {
        let short_hash = parts[0]; // last part

        if !short_hash.starts_with('g') {
            return Err(ShadowError::String(
                "git describe result error,expect commit hash end with:-g<hash>".to_string(),
            ));
        }
        let short_hash = short_hash.trim_start_matches('g');

        // Full example：v1.0.0-alpha0-5-ga1b2c3d
        let num_commits_str = parts[1];
        let num_commits = num_commits_str
            .parse::<usize>()
            .map_err(|e| ShadowError::String(e.to_string()))?;
        let last_tag = parts[2..]
            .iter()
            .rev()
            .copied()
            .collect::<Vec<_>>()
            .join("-");
        return Ok((last_tag, Some(num_commits), Some(short_hash.to_string())));
    }
    Ok((describe.to_string(), None, None))
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
                .env("GIT_OPTIONAL_LOCKS", "0")
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
            assert!(!v.desc.is_empty());
            if !k.eq(TAG)
                && !k.eq(LAST_TAG)
                && !k.eq(COMMITS_SINCE_TAG)
                && !k.eq(BRANCH)
                && !k.eq(GIT_STATUS_FILE)
            {
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
    fn test_parse_git_describe() {
        let commit_hash = "24skp4489";
        let describe = "v1.0.0";
        assert_eq!(
            parse_git_describe("v1.0.0", describe).unwrap(),
            (describe.into(), None, None)
        );

        let describe = "v1.0.0-0-g24skp4489";
        assert_eq!(
            parse_git_describe("v1.0.0", describe).unwrap(),
            ("v1.0.0".into(), Some(0), Some(commit_hash.into()))
        );

        let describe = "v1.0.0-1-g24skp4489";
        assert_eq!(
            parse_git_describe("v1.0.0", describe).unwrap(),
            ("v1.0.0".into(), Some(1), Some(commit_hash.into()))
        );

        let describe = "v1.0.0-alpha-0-g24skp4489";
        assert_eq!(
            parse_git_describe("v1.0.0-alpha", describe).unwrap(),
            ("v1.0.0-alpha".into(), Some(0), Some(commit_hash.into()))
        );

        let describe = "v1.0.0.alpha-0-g24skp4489";
        assert_eq!(
            parse_git_describe("v1.0.0.alpha", describe).unwrap(),
            ("v1.0.0.alpha".into(), Some(0), Some(commit_hash.into()))
        );

        let describe = "v1.0.0-alpha";
        assert_eq!(
            parse_git_describe("v1.0.0-alpha", describe).unwrap(),
            ("v1.0.0-alpha".into(), None, None)
        );

        let describe = "v1.0.0-alpha-99-0-g24skp4489";
        assert_eq!(
            parse_git_describe("v1.0.0-alpha-99", describe).unwrap(),
            ("v1.0.0-alpha-99".into(), Some(0), Some(commit_hash.into()))
        );

        let describe = "v1.0.0-alpha-99-024skp4489";
        assert!(parse_git_describe("v1.0.0-alpha-99", describe).is_err());

        let describe = "v1.0.0-alpha-024skp4489";
        assert!(parse_git_describe("v1.0.0-alpha", describe).is_err());

        let describe = "v1.0.0-alpha-024skp4489";
        assert!(parse_git_describe("v1.0.0-alpha", describe).is_err());

        let describe = "v1.0.0-alpha-g024skp4489";
        assert!(parse_git_describe("v1.0.0-alpha", describe).is_err());

        let describe = "v1.0.0----alpha-g024skp4489";
        assert!(parse_git_describe("v1.0.0----alpha", describe).is_err());
    }

    #[test]
    fn test_commit_date_timezone_preservation() {
        use crate::DateTime;

        // Test timezone-aware parsing
        let iso_date = "2021-08-04T12:34:03+08:00";
        let date_time = DateTime::from_iso8601_string(iso_date).unwrap();
        assert_eq!(date_time.human_format(), "2021-08-04 12:34:03 +08:00");
        assert!(date_time.to_rfc3339().contains("+08:00"));

        // Test UTC timezone
        let iso_date_utc = "2021-08-04T12:34:03Z";
        let date_time_utc = DateTime::from_iso8601_string(iso_date_utc).unwrap();
        assert_eq!(date_time_utc.human_format(), "2021-08-04 12:34:03 +00:00");

        // Test negative timezone
        let iso_date_neg = "2021-08-04T12:34:03-05:00";
        let date_time_neg = DateTime::from_iso8601_string(iso_date_neg).unwrap();
        assert_eq!(date_time_neg.human_format(), "2021-08-04 12:34:03 -05:00");
    }
}
