//! Inter-struct provides various derive macros to implement traits between arbitrary structs.
//!
//! The current available `derive` macros are:
//!
//! - `StructMerge`
//! - `StructMergeRef`
//! - `StructInto`
//! - `StructDefault`
//!
//! The general way to use such a derive macro is like this:
//!
//! ```rs,ignore
//! #[derive(StructInto)]
//! #[struct_into(["crate::path_to::TargetStruct"])]
//! pub struct Test {
//!     pub test: String,
//! }
//! ```
//!
//! This example generates an `impl Into<TargetStruct> for Test`, which converts `Test`
//! into some `TargetStruct`.
//!
//! Note that the target struct's paths has to be
//! - contained in this crate.
//! - relative to the current crate.
//!
//! Either a single path or a list of paths can be specified.
//! The traits will then be implemented for each given target struct.
//!
//! ```rs,ignore
//! #[struct_into("crate::path_to::TargetStruct")]
//! // or
//! #[struct_into(["crate::path_to::TargetStruct", "crate::path_to::AnotherTargetStruct"])]
//! ```
//!
//! Each derive macro can have their own options, so please check the individual docs for each
//! derive macro in this crate.

pub use inter_struct_codegen::*;

/// Docs and traits for struct merging.
pub mod merge;

/// Imports all modules to get you started.
pub mod prelude {
    pub use super::merge::*;
    pub use inter_struct_codegen::*;
}
