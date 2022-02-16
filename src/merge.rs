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
//!
//! ## Merge behavior of `merge_soft` and `merge_ref_soft`
//!
//! #### Target is not Optional
//!
//! As long as a target field is not optional **it won't be touched**!
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
//! This will wrap `src.test` into an `Option` and merge it into `target.test` but only if `target.test` is `None`: \
//! ```rust,ignore
//! if target.test.is_none() {
//!     target.test = Some(src.test);
//! }
//! ```
//!
//! #### Both are Optional
//!
//! ```rust,ignore
//! struct Src {
//!     test: Option<T>
//! }
//! struct Target {
//!     test: Option<T>
//! }
//! ```
//!
//! This will only merge `src.test` into `target.test` if `target.test` is `None`: \
//! ```rust,ignore
//! if target.test.is_none() {
//!     target.test = src.test;
//! }
//! ```

/// Merge another struct into `Self`.
pub trait StructMerge<Src> {
    /// Merge the given struct into `Self` whilst consuming it.
    fn merge(&mut self, src: Src);

    /// Merge the given struct into `Self` whilst consuming it.
    ///
    /// Nearly the same as `merge`, but any `Self::Option<T>` fields will only get merged if the
    /// value of the field is `None`.
    ///
    /// For example:
    /// ```ignore
    /// struct Target { a: Option<String> };
    /// struct Src { a: String };
    ///
    /// let target = Target { a: Some("test".to_string()) };
    /// let src = Src { a: "test2".to_string() };
    ///
    /// target.merge_soft(src);
    /// // Value didn't get merged as `target.a` was `Some`
    /// assert_eq!(target.a, "test".to_string());
    /// ```
    fn merge_soft(&mut self, src: Src);
}

/// Counterpart of [StructMerge].
/// This will merge `Self` into a given target.
pub trait StructMergeInto<Target: ?Sized> {
    /// Check the [StructMerge::merge] docs.
    fn merge_into(self, target: &mut Target);

    /// Check the [StructMerge::merge_soft] docs.
    fn merge_into_soft(self, target: &mut Target);
}

/// Implement the [StructMerge] trait for all types that provide [StructMergeInto] for it.
impl<Target, Src: StructMergeInto<Target>> StructMerge<Src> for Target {
    fn merge(&mut self, src: Src) {
        src.merge_into(self);
    }

    fn merge_soft(&mut self, src: Src) {
        src.merge_into_soft(self);
    }
}

/// Merge another borrowed struct into `Self`.
///
/// All fields to be merged on the borrowed struct have to implement [Clone].
pub trait StructMergeRef<Src> {
    /// Merge the given struct into `Self`.
    fn merge_ref(&mut self, src: &Src);

    /// Merge the given struct into `Self`.
    ///
    /// Nearly the same as `merge_ref`, but any `Self::Option<T>` fields will only get merged if the
    /// value of the field is `None`.
    ///
    /// For example:
    /// ```ignore
    /// struct Target { a: Option<String> };
    /// struct Src { a: String };
    ///
    /// let target = Target { a: Some("test".to_string()) };
    /// let src = Src { a: "test2".to_string() };
    ///
    /// target.merge_ref_soft(&src);
    /// // Value didn't get merged as `target.a` was `Some`
    /// assert_eq!(target.a, "test".to_string());
    /// ```
    fn merge_ref_soft(&mut self, src: &Src);
}

/// Counterpart of [StructMergeRef].
/// This will merge `&Self` into a given target.
pub trait StructMergeIntoRef<Target: ?Sized> {
    /// Check the [StructMergeRef::merge_ref] docs.
    fn merge_into_ref(&self, target: &mut Target);

    /// Check the [StructMergeRef::merge_ref_soft] docs.
    fn merge_into_ref_soft(&self, target: &mut Target);
}

/// Implement the [StructMergeRef] trait for all types that provide [StructMergeInto] for it.
impl<Target, Src: StructMergeIntoRef<Target>> StructMergeRef<Src> for Target {
    fn merge_ref(&mut self, src: &Src) {
        src.merge_into_ref(self);
    }

    fn merge_ref_soft(&mut self, src: &Src) {
        src.merge_into_ref_soft(self);
    }
}
