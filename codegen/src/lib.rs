mod error;
mod generate;
mod helper;
mod module;
mod parse;

#[cfg(feature = "debug")]
mod debug;

use proc_macro::TokenStream;

/// Implement the `Into` trait on this struct.
///
/// `struct.rs`
/// ```rust, ignore
/// use inter_struct::prelude::*;
///
/// pub struct Target {
///     pub test: String,
/// }
///
/// #[derive(StructInto)]
/// #[struct_into(["crate::structs::Target"])]
/// pub struct Test {
///     pub test: String,
/// }
/// ```
#[proc_macro_derive(StructInto, attributes(struct_into))]
pub fn struct_into(struct_ast: TokenStream) -> TokenStream {
    generate::into::struct_into_inner(struct_ast)
}

/// Implement the `Into` trait on this struct with `Default::default` for missing fields.
///
/// `struct.rs`
/// ```rust, ignore
/// use inter_struct::prelude::*;
///
/// pub struct Target {
///     pub test: String,
/// }
///
/// #[derive(StructIntoDefault)]
/// #[struct_into_default(["crate::structs::Target"])]
/// pub struct Test {
///     pub test: String,
/// }
/// ```
///
#[proc_macro_derive(StructIntoDefault, attributes(struct_into_default))]
pub fn struct_into_default(struct_ast: TokenStream) -> TokenStream {
    generate::into::struct_into_default_inner(struct_ast)
}

/// Implement the `StructMerge` trait on this struct.
///
/// `struct.rs`
/// ```rust, ignore
/// use inter_struct::prelude::*;
///
/// pub struct Target {
///     pub test: String,
/// }
///
/// #[derive(StructMerge)]
/// #[struct_merge("crate::structs::Target")]
/// pub struct Test {
///     pub test: String,
/// }
/// ```
#[proc_macro_derive(StructMerge, attributes(struct_merge))]
pub fn struct_merge(struct_ast: TokenStream) -> TokenStream {
    generate::merge::struct_merge_inner(struct_ast)
}

/// Implement the `StructMergeRef` trait on this struct.
///
/// `struct.rs`
/// ```rust, ignore
/// use inter_struct::prelude::*;
///
/// pub struct Target {
///     pub test: String,
/// }
///
/// #[derive(StructMergeRef)]
/// #[struct_merge_ref(["crate::structs::Target"])]
/// pub struct Test {
///     pub test: String,
/// }
/// ```
#[proc_macro_derive(StructMergeRef, attributes(struct_merge_ref))]
pub fn struct_merge_ref(struct_ast: TokenStream) -> TokenStream {
    generate::merge::struct_merge_ref_inner(struct_ast)
}
