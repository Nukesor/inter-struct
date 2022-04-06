use error::err;
use generate::generate_impl;
use module::get_struct_from_path;
use path::{get_root_src_path, parse_input_paths};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, Attribute, Expr, ItemStruct, Path};

mod error;
mod generate;
mod module;
mod path;

#[cfg(feature = "debug")]
mod debug;

pub(crate) struct Parameters {
    pub src_struct: ItemStruct,
    pub target_path: Path,
    pub target_struct: ItemStruct,
}

/// This enum is used to differentiate between the different implementations of the InterStruct
/// derive macro.
enum Mode {
    Merge,
    MergeRef,
    Into,
    IntoDefault,
}

/// Implement the StructMerge trait on this struct.
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
///
/// A target struct's paths has to be
/// - contained in this crate.
/// - relative to the current crate.
///
/// Eiter a single path or a list of paths can be specified.
/// The traits will then be implemented for each given target struct.
#[proc_macro_derive(StructMerge, attributes(struct_merge))]
pub fn struct_merge(struct_ast: TokenStream) -> TokenStream {
    // Parse the main macro input as a struct.
    // We work on a clone of the struct ast.
    // That way we don't have to parse it again, when we return it lateron.
    let src_struct = parse_macro_input!(struct_ast as ItemStruct);

    // Check if we can find the src root path of this crate.
    // Return early if it doesn't exist.
    let src_root_path = match get_root_src_path(&src_struct) {
        Ok(path) => path,
        Err(err) => return TokenStream::from(err),
    };

    // Check if we can find the src root path of this crate.
    // Return early if it doesn't exist.
    let attribute = match parse_attribute(&src_struct, "StructMerge", "struct_merge") {
        Ok(attribute) => attribute,
        Err(err) => return TokenStream::from(err),
    };

    let attribute_args = TokenStream::from(attribute.tokens.clone());
    let parsed_args = parse_macro_input!(attribute_args as Expr);

    let impls = inter_struct_base(&src_root_path, &src_struct, parsed_args, Mode::Merge);

    // Merge all generated pieces of the code with the original unaltered struct.
    let mut tokens = TokenStream::new();
    tokens.extend(impls.into_iter().map(TokenStream::from));

    #[cfg(feature = "debug")]
    println!("StructMerge impl: {}", tokens.to_string());

    tokens
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
///
/// A target struct's paths has to be
/// - contained in this crate.
/// - relative to the current crate.
///
/// Eiter a single path or a list of paths can be specified.
/// The traits will then be implemented for each given target struct.
#[proc_macro_derive(StructMergeRef, attributes(struct_merge_ref))]
pub fn struct_merge_ref(struct_ast: TokenStream) -> TokenStream {
    // Parse the main macro input as a struct.
    // We work on a clone of the struct ast.
    // That way we don't have to parse it again, when we return it lateron.
    let src_struct = parse_macro_input!(struct_ast as ItemStruct);

    // Check if we can find the src root path of this crate.
    // Return early if it doesn't exist.
    let src_root_path = match get_root_src_path(&src_struct) {
        Ok(path) => path,
        Err(err) => return TokenStream::from(err),
    };

    // Check if we can find the src root path of this crate.
    // Return early if it doesn't exist.
    let attribute = match parse_attribute(&src_struct, "StructMergeRef", "struct_merge_ref") {
        Ok(attribute) => attribute,
        Err(err) => return TokenStream::from(err),
    };

    let attribute_args = TokenStream::from(attribute.tokens.clone());
    let parsed_args = parse_macro_input!(attribute_args as Expr);

    let impls = inter_struct_base(&src_root_path, &src_struct, parsed_args, Mode::MergeRef);

    // Merge all generated pieces of the code with the original unaltered struct.
    let mut tokens = TokenStream::new();
    tokens.extend(impls.into_iter().map(TokenStream::from));

    #[cfg(feature = "debug")]
    println!("StructMergeRef impl: {}", tokens.to_string());

    tokens
}

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
///
/// A target struct's paths has to be
/// - contained in this crate.
/// - relative to the current crate.
///
/// Eiter a single path or a list of paths can be specified.
/// The traits will then be implemented for each given target struct.
#[proc_macro_derive(StructInto, attributes(struct_into))]
pub fn struct_into(struct_ast: TokenStream) -> TokenStream {
    // Parse the main macro input as a struct.
    // We work on a clone of the struct ast.
    // That way we don't have to parse it again, when we return it lateron.
    let src_struct = parse_macro_input!(struct_ast as ItemStruct);

    // Check if we can find the src root path of this crate.
    // Return early if it doesn't exist.
    let src_root_path = match get_root_src_path(&src_struct) {
        Ok(path) => path,
        Err(err) => return TokenStream::from(err),
    };

    // Check if we can find the src root path of this crate.
    // Return early if it doesn't exist.
    let attribute = match parse_attribute(&src_struct, "StructInto", "struct_into") {
        Ok(attribute) => attribute,
        Err(err) => return TokenStream::from(err),
    };

    let attribute_args = TokenStream::from(attribute.tokens.clone());
    let parsed_args = parse_macro_input!(attribute_args as Expr);

    let impls = inter_struct_base(&src_root_path, &src_struct, parsed_args, Mode::Into);

    // Merge all generated pieces of the code with the original unaltered struct.
    let mut tokens = TokenStream::new();
    tokens.extend(impls.into_iter().map(TokenStream::from));

    #[cfg(feature = "debug")]
    println!("StructInto impl: {}", tokens.to_string());

    tokens
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
/// A target struct's paths has to be
/// - contained in this crate.
/// - relative to the current crate.
///
/// Eiter a single path or a list of paths can be specified.
/// The traits will then be implemented for each given target struct.
#[proc_macro_derive(StructIntoDefault, attributes(struct_into_default))]
pub fn struct_into_default(struct_ast: TokenStream) -> TokenStream {
    // Parse the main macro input as a struct.
    // We work on a clone of the struct ast.
    // That way we don't have to parse it again, when we return it lateron.
    let src_struct = parse_macro_input!(struct_ast as ItemStruct);

    // Check if we can find the src root path of this crate.
    // Return early if it doesn't exist.
    let src_root_path = match get_root_src_path(&src_struct) {
        Ok(path) => path,
        Err(err) => return TokenStream::from(err),
    };

    // Check if we can find the src root path of this crate.
    // Return early if it doesn't exist.
    let attribute = match parse_attribute(&src_struct, "StructInto", "struct_into_default") {
        Ok(attribute) => attribute,
        Err(err) => return TokenStream::from(err),
    };

    let attribute_args = TokenStream::from(attribute.tokens.clone());
    let parsed_args = parse_macro_input!(attribute_args as Expr);

    let impls = inter_struct_base(&src_root_path, &src_struct, parsed_args, Mode::IntoDefault);

    // Merge all generated pieces of the code with the original unaltered struct.
    let mut tokens = TokenStream::new();
    tokens.extend(impls.into_iter().map(TokenStream::from));

    #[cfg(feature = "debug")]
    println!("StructInto impl: {}", tokens.to_string());

    tokens
}

fn parse_attribute(
    src_struct: &ItemStruct,
    derive_name: &str,
    name: &str,
) -> Result<Attribute, TokenStream2> {
    for attribute in src_struct.attrs.iter() {
        let path = &attribute.path;

        // Make sure we don't have a multi-segment path in our attribute.
        // It's probably an attribute from a different macro.
        if path.segments.len() != 1 {
            continue;
        }

        let attribute_name = path.segments.first().unwrap().ident.to_string();
        if name == attribute_name.as_str() {
            return Ok(attribute.clone());
        }
    }

    Err(err!(
        src_struct,
        "{} requires the '{}' attribute.",
        derive_name,
        name
    ))
}

fn inter_struct_base(
    src_root_path: &std::path::Path,
    src_struct: &ItemStruct,
    parsed_args: Expr,
    mode: Mode,
) -> Vec<TokenStream2> {
    // Get the input paths from the given argument expressions.
    let paths = parse_input_paths(parsed_args);
    let paths = match paths {
        Ok(paths) => paths,
        Err(err) => return vec![err],
    };

    // Go through all paths and process the respective struct.
    let mut impls = Vec::new();
    for target_path in paths {
        // Make sure we found the struct at that path.
        let target_struct =
            match get_struct_from_path(src_root_path.to_path_buf(), target_path.clone()) {
                Ok(ast) => ast,
                Err(error) => {
                    impls.push(error);
                    continue;
                }
            };

        let params = Parameters {
            src_struct: src_struct.clone(),
            target_path,
            target_struct,
        };

        // Generate the MergeStruct trait implementations.
        match generate_impl(&mode, params) {
            Ok(ast) => impls.push(ast),
            Err(error) => {
                impls.push(error);
                continue;
            }
        }
    }

    impls
}
