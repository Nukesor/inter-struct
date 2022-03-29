use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Field;

use crate::error::*;
use crate::generate::field::*;
use crate::generate::types::*;
use crate::Parameters;

/// Generate the [std::convert::From] for given structs.
pub(crate) fn impl_into_default(params: &Parameters, fields: Vec<(Field, Field)>) -> TokenStream {
    let mut initializer_tokens = TokenStream::new();

    // Add `into_default` impl.
    let stream = into_default(params, fields);
    initializer_tokens.extend(vec![stream]);

    // Surround the function with the correct Default  `impl` block.
    let src_ident = &params.src_struct.ident;
    let target_path = &params.target_path;
    let test = quote! {
        impl std::convert::From<#src_ident> for #target_path {
            fn from(src: #src_ident) -> Self {
                #initializer_tokens
            }
        }
    };
    test
}

/// Generate the [std::convert::From] function body for given structs.
fn into_default(params: &Parameters, fields: Vec<(Field, Field)>) -> TokenStream {
    let mut assignments = TokenStream::new();

    for (src_field, dest_field) in fields {
        let src_field_ident = src_field.ident;
        let dest_field_ident = dest_field.ident;

        // Find out, whether the fields are optional or not.
        let src_field_type = match determine_field_type(src_field.ty) {
            Ok(field) => field,
            Err(err) => {
                assignments.extend(vec![err]);
                continue;
            }
        };
        let target_field_type = match determine_field_type(dest_field.ty) {
            Ok(field) => field,
            Err(err) => {
                assignments.extend(vec![err]);
                continue;
            }
        };

        let snippet = match (src_field_type, target_field_type) {
            // Both fields have the same type
            (FieldType::Normal(src_type), FieldType::Normal(target_type)) => {
                Some(equal_type_or_err!(
                    src_type,
                    target_type,
                    "",
                    quote! {
                        #dest_field_ident: src.#src_field_ident,
                    }
                ))
            }
            // The src is optional and needs to be `Some(T)` to be merged.
            (FieldType::Optional { .. }, FieldType::Normal(_)) => None,
            // The target is optional and needs to be wrapped in `Some(T)` to be merged.
            (
                FieldType::Normal(src_type),
                FieldType::Optional {
                    inner: target_type, ..
                },
            ) => Some(equal_type_or_err!(
                src_type,
                target_type,
                "",
                quote! {
                    #dest_field_ident: Some(src.#src_field_ident),
                }
            )),
            // Both fields are optional. It can now be either of these:
            // - (Option<T>, Option<T>)
            // - (Option<Option<T>>, Option<T>)
            // - (Option<T>, Option<Option<T>>)
            (
                FieldType::Optional {
                    inner: inner_src_type,
                    outer: outer_src_type,
                },
                FieldType::Optional {
                    inner: inner_target_type,
                    outer: outer_target_type,
                },
            ) => {
                // Handling the (Option<T>, Option<T>) case
                if is_equal_type(&inner_src_type, &inner_target_type) {
                    Some(quote! {
                        #dest_field_ident: src.#src_field_ident,
                    })
                // Handling the (src: Option<Option<<T>>, dest: Option<T>) case
                } else if is_equal_type(&inner_src_type, &outer_target_type) {
                    None
                // Handling the (src: Option<<T>, dest: Option<Option<T>)> case
                } else {
                    Some(equal_type_or_err!(
                        outer_src_type,
                        inner_target_type,
                        "",
                        quote! {
                            #dest_field_ident: Some(src.#src_field_ident.clone()),
                        }
                    ))
                }
            }
            // Skip anything where either of the fields are invalid
            (FieldType::Invalid, _) | (_, FieldType::Invalid) => None,
        };

        if let Some(snippet) = snippet {
            assignments.extend(vec![snippet]);
        }
    }

    let target_path = &params.target_path;
    let assignment_code = assignments.to_token_stream();
    quote! {
            #target_path {
                #assignment_code
                ..#target_path::default()
            }
    }
}
