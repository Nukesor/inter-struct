//! # Merge Behavior
//!
//! The following will explain the merge behavior on the example of a single field.
//!
//! ## Merge behavior of `merge` and `merge_ref`
//!
//! #### Same Type
//!
//! ```rust,ignore
//! struct Src {
//!     test: T
//! }
//! struct Target {
//!     test: T
//! }
//! ```
//!
//! This will simply merge `src.test` into `target.test`: \
//! `target.test = src.test`
//!
//! #### Target is Optional
//!
//! ```rust,ignore
//! struct Src {
//!     test: T
//! }
//! struct Target {
//!     test: Option<T>
//! }
//! ```
//!
//! This will wrap `src.test` into an `Option` and merge it into `target.test`: \
//! `target.test = Some(src.test);`
//!
//! #### Source is Optional
//!
//! ```rust,ignore
//! struct Src {
//!     test: Option<T>
//! }
//! struct Target {
//!     test: T
//! }
//! ```
//!
//! This will only merge `src.test` into `target.test` if `src.test` is `Some`: \
//! ```rust,ignore
//! if let Some(value) = src.test {
//!     target.test = value;
//! }
//! ```

/// Merge another struct into `Self`.
pub trait StructMerge<Src> {
    /// Merge the given struct into `Self` whilst consuming it.
    fn merge(&mut self, src: Src);
}

/// Counterpart of [StructMerge].
/// This will merge `Self` into a given target.
pub trait StructMergeInto<Target: ?Sized> {
    /// Check the [StructMerge::merge] docs.
    fn merge_into(self, target: &mut Target);
}

/// Implement the [StructMerge] trait for all types that provide [StructMergeInto] for it.
impl<Target, Src: StructMergeInto<Target>> StructMerge<Src> for Target {
    fn merge(&mut self, src: Src) {
        src.merge_into(self);
    }
}

/// Merge another borrowed struct into `Self`.
///
/// All fields to be merged on the borrowed struct have to implement [Clone].
pub trait StructMergeRef<Src> {
    /// Merge the given struct into `Self`.
    fn merge_ref(&mut self, src: &Src);
}

/// Counterpart of [StructMergeRef].
/// This will merge `&Self` into a given target.
pub trait StructMergeIntoRef<Target: ?Sized> {
    /// Check the [StructMergeRef::merge_ref] docs.
    fn merge_into_ref(&self, target: &mut Target);
}

/// Implement the [StructMergeRef] trait for all types that provide [StructMergeInto] for it.
impl<Target, Src: StructMergeIntoRef<Target>> StructMergeRef<Src> for Target {
    fn merge_ref(&mut self, src: &Src) {
        src.merge_into_ref(self);
    }
}
