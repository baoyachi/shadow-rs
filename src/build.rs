use crate::date_time::DEFINE_SOURCE_DATE_EPOCH;
use crate::hook::HookExt;
use crate::{default_deny, SdResult, Shadow, DEFINE_SHADOW_RS};
use is_debug::is_debug;
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};

/// `shadow-rs` build constant identifiers.
pub type ShadowConst = &'static str;

/// Serialized values for build constants.
#[derive(Debug, Clone)]
pub struct ConstVal {
    /// User-facing documentation for the build constant.
    pub desc: String,
    /// Serialized value of the build constant.
    pub v: String,
    /// Type of the build constant.
    pub t: ConstType,
}

impl ConstVal {
    pub fn new<S: Into<String>>(desc: S) -> ConstVal {
        // Creates a new `ConstVal` with an empty string as its value and `Str` as its type.
        ConstVal {
            desc: desc.into(),
            v: "".to_string(),
            t: ConstType::Str,
        }
    }

    pub fn new_bool<S: Into<String>>(desc: S) -> ConstVal {
        // Creates a new `ConstVal` with "true" as its value and `Bool` as its type.
        ConstVal {
            desc: desc.into(),
            v: "true".to_string(),
            t: ConstType::Bool,
        }
    }

    pub fn new_slice<S: Into<String>>(desc: S) -> ConstVal {
        // Creates a new `ConstVal` with an empty string as its value and `Slice` as its type.
        ConstVal {
            desc: desc.into(),
            v: "".to_string(),
            t: ConstType::Slice,
        }
    }
}

/// Supported types of build constants.
#[derive(Debug, Clone)]
pub enum ConstType {
    /// [`&str`](`str`).
    Str,
    /// [`bool`].
    Bool,
    /// [`&[u8]`].
    Slice,
}

impl Display for ConstType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstType::Str => write!(f, "&str"),
            ConstType::Bool => write!(f, "bool"),
            ConstType::Slice => write!(f, "&[u8]"),
        }
    }
}

/// The BuildPattern enum defines strategies for triggering package rebuilding.
///
/// Default mode is `Lazy`.
///
/// * `Lazy`: The lazy mode. In this mode, if the current Rust environment is set to `debug`,
///   the rebuild package will not run every time the build script is triggered.
///   If the environment is set to `release`, it behaves the same as the `RealTime` mode.
/// * `RealTime`: The real-time mode. It will always trigger rebuilding a package upon any change,
///   regardless of whether the Rust environment is set to `debug` or `release`.
/// * `Custom`: The custom build mode, an enhanced version of `RealTime` mode, allowing for user-defined conditions
///   to trigger rebuilding a package.
///
#[derive(Debug, Default, Clone)]
pub enum BuildPattern {
    #[default]
    Lazy,
    RealTime,
    Custom {
        /// A list of paths that, if changed, will trigger a rebuild.
        /// See <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed>
        if_path_changed: Vec<String>,
        /// A list of environment variables that, if changed, will trigger a rebuild.
        /// See <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-env-changed>
        if_env_changed: Vec<String>,
    },
}

impl BuildPattern {
    /// Determines when Cargo should rerun the build script based on the configured pattern.
    ///
    /// # Arguments
    ///
    /// * `other_keys` - An iterator over additional keys that should trigger a rebuild if they change.
    /// * `out_dir` - The output directory where generated files are placed.
    pub(crate) fn rerun_if<'a>(
        &self,
        other_keys: impl Iterator<Item = &'a ShadowConst>,
        out_dir: &str,
    ) {
        match self {
            BuildPattern::Lazy => {
                if is_debug() {
                    return;
                }
            }
            BuildPattern::RealTime => {}
            BuildPattern::Custom {
                if_path_changed,
                if_env_changed,
            } => {
                if_env_changed
                    .iter()
                    .for_each(|key| println!("cargo:rerun-if-env-changed={key}"));
                if_path_changed
                    .iter()
                    .for_each(|p| println!("cargo:rerun-if-changed={p}"));
            }
        }

        other_keys.for_each(|key| println!("cargo:rerun-if-env-changed={key}"));
        println!("cargo:rerun-if-env-changed={}", DEFINE_SOURCE_DATE_EPOCH);
        println!("cargo:rerun-if-changed={}/{}", out_dir, DEFINE_SHADOW_RS);
    }
}

/// A builder pattern structure to construct a `Shadow` instance.
///
/// This struct allows for configuring various aspects of how shadow-rs will be built into your Rust project.
/// It provides methods to set up hooks, specify build patterns, define paths, and deny certain build constants.
///
/// # Fields
///
/// * `hook`: An optional hook that can be used during the build process. Hooks implement the `HookExt` trait.
/// * `build_pattern`: Determines the strategy for triggering package rebuilds (`Lazy`, `RealTime`, or `Custom`).
/// * `deny_const`: A set of build constant identifiers that should not be included in the build.
/// * `src_path`: The source path from which files are read for building.
/// * `out_path`: The output path where generated files will be placed.
///
pub struct ShadowBuilder<'a> {
    hook: Option<Box<dyn HookExt + 'a>>,
    build_pattern: BuildPattern,
    deny_const: BTreeSet<ShadowConst>,
    src_path: Option<String>,
    out_path: Option<String>,
}

impl<'a> ShadowBuilder<'a> {
    /// Creates a new `ShadowBuilder` with default settings.
    ///
    /// Initializes the builder with the following defaults:
    /// - `hook`: None
    /// - `build_pattern`: `BuildPattern::Lazy`
    /// - `deny_const`: Uses the result from `default_deny()`
    /// - `src_path`: Attempts to get the manifest directory using `CARGO_MANIFEST_DIR` environment variable.
    /// - `out_path`: Attempts to get the output directory using `OUT_DIR` environment variable.
    ///
    /// # Returns
    ///
    /// A new instance of `ShadowBuilder`.
    pub fn builder() -> Self {
        let default_src_path = std::env::var("CARGO_MANIFEST_DIR").ok();
        let default_out_path = std::env::var("OUT_DIR").ok();
        Self {
            hook: None,
            build_pattern: BuildPattern::default(),
            deny_const: default_deny(),
            src_path: default_src_path,
            out_path: default_out_path,
        }
    }

    /// Sets the build hook for this builder.
    ///
    /// # Arguments
    ///
    /// * `hook` - An object implementing the `HookExt` trait that defines custom behavior for the build process.
    ///
    /// # Returns
    ///
    /// A new `ShadowBuilder` instance with the specified hook applied.
    pub fn hook(mut self, hook: impl HookExt + 'a) -> Self {
        self.hook = Some(Box::new(hook));
        self
    }

    /// Sets the source path for this builder.
    ///
    /// # Arguments
    ///
    /// * `src_path` - A string reference that specifies the source directory for the build.
    ///
    /// # Returns
    ///
    /// A new `ShadowBuilder` instance with the specified source path.
    pub fn src_path<P: AsRef<str>>(mut self, src_path: P) -> Self {
        self.src_path = Some(src_path.as_ref().to_owned());
        self
    }

    /// Sets the output path for this builder.
    ///
    /// # Arguments
    ///
    /// * `out_path` - A string reference that specifies the output directory for the build.
    ///
    /// # Returns
    ///
    /// A new `ShadowBuilder` instance with the specified output path.
    pub fn out_path<P: AsRef<str>>(mut self, out_path: P) -> Self {
        self.out_path = Some(out_path.as_ref().to_owned());
        self
    }

    /// Sets the build pattern for this builder.
    ///
    /// # Arguments
    ///
    /// * `pattern` - A `BuildPattern` that determines when the package should be rebuilt.
    ///
    /// # Returns
    ///
    /// A new `ShadowBuilder` instance with the specified build pattern.
    pub fn build_pattern(mut self, pattern: BuildPattern) -> Self {
        self.build_pattern = pattern;
        self
    }

    /// Sets the denied constants for this builder.
    ///
    /// # Arguments
    ///
    /// * `deny_const` - A set of `ShadowConst` that should be excluded from the build.
    ///
    /// # Returns
    ///
    /// A new `ShadowBuilder` instance with the specified denied constants.
    pub fn deny_const(mut self, deny_const: BTreeSet<ShadowConst>) -> Self {
        self.deny_const = deny_const;
        self
    }

    /// Builds a `Shadow` instance based on the current configuration.
    ///
    /// # Returns
    ///
    /// A `SdResult<Shadow>` that represents the outcome of the build operation.
    pub fn build(self) -> SdResult<Shadow> {
        Shadow::build_inner(self)
    }

    /// Gets the source path if it has been set.
    ///
    /// # Returns
    ///
    /// A `SdResult<&String>` containing the source path or an error if the path is missing.
    pub fn get_src_path(&self) -> SdResult<&String> {
        let src_path = self.src_path.as_ref().ok_or("missing `src_path`")?;
        Ok(src_path)
    }

    /// Gets the output path if it has been set.
    ///
    /// # Returns
    ///
    /// A `SdResult<&String>` containing the output path or an error if the path is missing.
    pub fn get_out_path(&self) -> SdResult<&String> {
        let out_path = self.out_path.as_ref().ok_or("missing `out_path`")?;
        Ok(out_path)
    }

    /// Gets the build pattern.
    ///
    /// # Returns
    ///
    /// A reference to the `BuildPattern` currently configured for this builder.
    pub fn get_build_pattern(&self) -> &BuildPattern {
        &self.build_pattern
    }

    /// Gets the denied constants.
    ///
    /// # Returns
    ///
    /// A reference to the set of `ShadowConst` that are denied for this build.
    pub fn get_deny_const(&self) -> &BTreeSet<ShadowConst> {
        &self.deny_const
    }

    /// Gets the build hook if it has been set.
    ///
    /// # Returns
    ///
    /// An option containing a reference to the hook if one is present.
    pub fn get_hook(&'a self) -> Option<&'a (dyn HookExt + 'a)> {
        self.hook.as_deref()
    }
}
