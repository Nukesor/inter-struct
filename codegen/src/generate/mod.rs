use proc_macro2::TokenStream;
use syn::Fields;
use syn::{Expr, ItemStruct, Path};

use crate::error::err;
use crate::module::get_struct_from_path;

/// Some helper functions and macros, that need to be declared before the actual generaction code.
mod field;
mod types;

pub mod into;
pub mod merge;

pub(crate) struct Parameters {
    pub src_struct: ItemStruct,
    pub target_path: Path,
    pub target_struct: ItemStruct,
}

/// This enum is used to differentiate between the different implementations of the InterStruct
/// derive macro.
pub(crate) enum Mode {
    Merge,
    MergeRef,
    Into,
    IntoDefault,
}

fn inter_struct_base(
    src_root_path: &std::path::Path,
    src_struct: &ItemStruct,
    parsed_args: Expr,
    mode: Mode,
) -> Vec<TokenStream> {
    // Get the input paths from the given argument expressions.
    let paths = crate::parse::input_paths(parsed_args);
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

/// Return a Tokenstream that contains the implementation for a given trait,
/// `src` and `target` struct.
///
/// Known Limitations:
/// - Error, when using different generic aliases that have same type.
/// - Visibility of the `target` struct isn't taken into account.
///     This might get better when module resolution is done properly.
/// - Type equality cannot be properly ensured at this stage.
///     The resulting code will still be correct though, as any type incompatibilities will be
///     caught by the compiler anyway.
pub(crate) fn generate_impl(mode: &Mode, params: Parameters) -> Result<TokenStream, TokenStream> {
    let target_fields = match params.target_struct.fields.clone() {
        Fields::Named(fields) => fields,
        _ => {
            return Err(err!(
                params.target_struct,
                "inter_struct only works on structs with named fields."
            ));
        }
    };
    let src_fields = match params.src_struct.fields.clone() {
        Fields::Named(fields) => fields,
        _ => {
            return Err(err!(
                params.target_struct,
                "inter_struct only works on structs with named fields."
            ));
        }
    };

    let mut similar_fields = Vec::new();
    for src_field in src_fields.named {
        let src_ident = src_field.ident.clone().unwrap();
        for target_field in target_fields.named.clone() {
            let target_ident = target_field.clone().ident.unwrap();
            if src_ident == target_ident {
                similar_fields.push((src_field.clone(), target_field));
            }
        }
    }

    // In the following, we'll generate all required functions for the `MergeInto` impl.
    // If any of the functions fails to be generated, we skip the impl for this struct.
    // The errors will be generated in the individual token generator functions.
    match *mode {
        Mode::Merge => Ok(merge::owned::impl_owned(&params, similar_fields)),
        Mode::MergeRef => Ok(merge::borrowed::impl_borrowed(&params, similar_fields)),
        Mode::Into => Ok(into::normal::impl_into(&params, similar_fields, false)),
        Mode::IntoDefault => Ok(into::normal::impl_into(&params, similar_fields, true)),
    }
}
