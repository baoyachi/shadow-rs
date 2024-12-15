use crate::hook::HookExt;
use crate::{default_deny, SdResult, Shadow};
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
        ConstVal {
            desc: desc.into(),
            v: "".to_string(),
            t: ConstType::Str,
        }
    }

    pub fn new_bool<S: Into<String>>(desc: S) -> ConstVal {
        ConstVal {
            desc: desc.into(),
            v: "true".to_string(),
            t: ConstType::Bool,
        }
    }

    pub fn new_slice<S: Into<String>>(desc: S) -> ConstVal {
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

pub struct ShadowBuilder<'a> {
    hook: Option<Box<dyn HookExt + 'a>>,
    build_pattern: BuildPattern,
    deny_const: BTreeSet<ShadowConst>,
    src_path: Option<String>,
    out_path: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub enum BuildPattern {
    //TODO handle pattern
    #[default]
    Lazy,
    RealTime,
    Custom(CBuildPattern),
}

#[derive(Debug, Default, Clone)]
pub struct CBuildPattern {
    // https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed
    if_path_changed: Vec<String>,
    // https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-env-changed
    if_env_changed: Vec<String>,
}

impl<'a> ShadowBuilder<'a> {
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

    pub fn hook(mut self, hook: impl HookExt + 'a) -> Self {
        self.hook = Some(Box::new(hook));
        self
    }

    pub fn src_path<P: AsRef<str>>(mut self, src_path: P) -> Self {
        self.src_path = Some(src_path.as_ref().to_owned());
        self
    }

    pub fn out_path<P: AsRef<str>>(mut self, out_path: P) -> Self {
        self.out_path = Some(out_path.as_ref().to_owned());
        self
    }

    pub fn build_pattern(mut self, pattern: BuildPattern) -> Self {
        self.build_pattern = pattern;
        self
    }

    pub fn deny_const(mut self, deny_const: BTreeSet<ShadowConst>) -> Self {
        self.deny_const = deny_const;
        self
    }

    pub fn build(self) -> SdResult<Shadow> {
        Shadow::build_inner(self)
    }

    pub fn get_src_path(&self) -> SdResult<&String> {
        let src_path = self.src_path.as_ref().ok_or("missing `src_path`")?;
        Ok(src_path)
    }
    pub fn get_out_path(&self) -> SdResult<&String> {
        let out_path = self.out_path.as_ref().ok_or("missing `out_path`")?;
        Ok(out_path)
    }

    pub fn get_build_pattern(&self) -> &BuildPattern {
        &self.build_pattern
    }

    pub fn get_deny_const(&self) -> &BTreeSet<ShadowConst> {
        &self.deny_const
    }

    pub fn get_hook(&'a self) -> Option<&Box<dyn HookExt + 'a>> {
        self.hook.as_ref()
    }
}
