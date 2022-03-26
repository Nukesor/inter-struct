use generate::generate_impl;
use module::get_struct_from_path;
use path::{get_root_src_path, parse_input_paths};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, Expr, ItemStruct, Path};

/// Helper macro, which attaches an error to a given span.
macro_rules! err {
    ($span:expr, $($text:expr),*) => {
        {
            #[allow(unused_imports)]
            use syn::spanned::Spanned;
            let message = format!($($text,)*);
            let span = $span.span();
            quote::quote_spanned!( span => compile_error!(#message); )
        }
    }
}

// Uncomment this as soon as proc_macro_diagnostic land in stable.
//
//#![feature(proc_macro_diagnostic)]
///// Helper macro, which attaches an error to a given span.
//macro_rules! err {
//    ($span:ident, $($text:expr),*) => {
//        $span.span()
//            .unwrap()
//            .error(format!($($text,)*))
//            .emit();
//    }
//}

/// Helper macro, which takes a result.
/// Ok(T) => simply return the T
/// Err(err) => Emits an compiler error on the given span with the provided error message.
///             Also returns early with `None`.
///             `None` is used throughout this crate as a gracefull failure.
///             That way all code that can be created is being generated and the user sees all
///             errors without the macro code panicking.
macro_rules! ok_or_err_return {
    ($expr:expr, $span:ident, $($text:expr),*) => {
        match $expr {
            Ok(result) => result,
            Err(error) =>  {
                return Err(err!($span, $($text,)* error));
            }
        }
    }
}

mod generate;
mod module;
mod path;

#[cfg(feature = "debug")]
mod debug;

/// This enum is used to differentiate between the different implementations of the InterStruct
/// derive macro.
enum Mode {
    Merge,
    MergeRef,
}

pub(crate) struct Parameters {
    pub src_struct: ItemStruct,
    pub target_path: Path,
    pub target_struct: ItemStruct,
}

/// Implement various InterStruct traits on this struct.
///
/// `struct.rs`
/// ```ignore
/// use inter_struct::prelude::*;
///
/// pub struct Target {
///     pub test: String,
/// }
///
/// pub struct OtherTarget {
///     pub test: String,
/// }
///
/// #[derive[InterStruct]]
/// #[merge_ref(["crate::structs::Target", "crate:structs::OtherTarget"])]
/// #[merge("crate::structs::Target"]
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
#[proc_macro_derive(InterStruct, attributes(merge, merge_ref))]
pub fn inter_struct(struct_ast: TokenStream) -> TokenStream {
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

    let mut all_impls = Vec::new();

    for attribute in src_struct.attrs.iter() {
        let path = &attribute.path;

        // Make sure we don't have a multi-segment path in our attribute.
        // It's probably an attribute from a different macro.
        if path.segments.len() != 1 {
            continue;
        }
        let attribute_name = path.segments.first().unwrap().ident.to_string();
        let mode = match attribute_name.as_str() {
            "merge" => Mode::Merge,
            "merge_ref" => Mode::MergeRef,
            _ => continue,
        };

        let attribute_args = TokenStream::from(attribute.tokens.clone());
        let parsed_args = parse_macro_input!(attribute_args as Expr);

        let mut impls = inter_struct_base(&src_root_path, &src_struct, parsed_args, mode);
        all_impls.append(&mut impls);
    }

    // Merge all generated pieces of the code with the original unaltered struct.
    let mut tokens = TokenStream::new();
    tokens.extend(all_impls.into_iter().map(TokenStream::from));

    tokens
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
