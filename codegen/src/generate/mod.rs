use proc_macro2::TokenStream;
use syn::Fields;

use crate::error::err;
use crate::{Mode, Parameters};

/// Some helper functions and macros, that need to be declared before the actual generaction code.
mod field;
mod types;

pub mod into;
pub mod merge;

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
        for target_field in target_fields.named.clone() {
            if src_field.ident == target_field.ident {
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
        Mode::Into => Ok(into::normal::impl_into(&params, similar_fields)),
        Mode::IntoDefault => Ok(into::with_default::impl_into_default(
            &params,
            similar_fields,
        )),
    }
}
