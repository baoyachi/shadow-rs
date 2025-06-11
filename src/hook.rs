use crate::{default_deny, SdResult, ShadowConst};
use std::collections::BTreeSet;
use std::fs::File;

/// A trait that extends the functionality of hooks.
/// It provides methods to get the default deny list and the inner hook function.
pub trait HookExt {
    /// Returns the default deny list.
    fn default_deny(&self) -> BTreeSet<ShadowConst>;

    /// Returns a reference to the inner hook function.
    fn hook_inner(&self) -> &dyn Fn(&File) -> SdResult<()>;
}

/// Implement the `HookExt` trait for any function that takes a `&File` and returns a `SdResult<()>`.
impl<F> HookExt for F
where
    F: Fn(&File) -> SdResult<()>,
{
    /// Returns the default deny list using the `default_deny` function from the crate.
    fn default_deny(&self) -> BTreeSet<ShadowConst> {
        default_deny()
    }

    /// Returns a reference to the function itself.
    fn hook_inner(&self) -> &dyn Fn(&File) -> SdResult<()> {
        self
    }
}

/// Implement the `HookExt` trait for a tuple containing a function and a deny list.
impl<F> HookExt for (F, BTreeSet<ShadowConst>)
where
    F: Fn(&File) -> SdResult<()>,
{
    /// Returns the deny list stored in the second element of the tuple.
    fn default_deny(&self) -> BTreeSet<ShadowConst> {
        self.1.clone()
    }

    /// Returns a reference to the function stored in the first element of the tuple.
    fn hook_inner(&self) -> &dyn Fn(&File) -> SdResult<()> {
        &self.0
    }
}
