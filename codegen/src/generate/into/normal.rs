use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Field;

use crate::error::*;
use crate::generate::field::*;
use crate::generate::types::*;
use crate::generate::*;

/// Generate the [std::convert::From] for given structs.
pub(crate) fn impl_into(
    params: &Parameters,
    fields: Vec<(Field, Field)>,
    default_impl: bool,
) -> TokenStream {
    let mut initializer_tokens = TokenStream::new();

    // Add `into` impl.
    let stream = into(params, fields, default_impl);
    initializer_tokens.extend(vec![stream]);

    // Surround the function with the correct Default  `impl` block.
    let src_ident = &params.src_struct.ident;
    let target_path = &params.target_path;
    quote! {
        impl std::convert::From<#src_ident> for #target_path {
            fn from(src: #src_ident) -> Self {
                #initializer_tokens
            }
        }
    }
}

/// Generate the [std::convert::From] function body for given structs.
fn into(params: &Parameters, fields: Vec<(Field, Field)>, default_impl: bool) -> TokenStream {
    let mut assignments = TokenStream::new();
    let mut errors = TokenStream::new();

    for (src_field, target_field) in fields {
        let src_field_ident = src_field.ident;
        let target_field_ident = target_field.ident;

        // Find out, whether the fields are optional or not.
        let src_field_type = match determine_field_type(src_field.ty) {
            Ok(field) => field,
            Err(err) => {
                assignments.extend(vec![err]);
                continue;
            }
        };
        let target_field_type = match determine_field_type(target_field.ty) {
            Ok(field) => field,
            Err(err) => {
                assignments.extend(vec![err]);
                continue;
            }
        };

        match (src_field_type, target_field_type) {
            // Both fields have the same type
            (FieldType::Normal(src_type), FieldType::Normal(target_type)) => {
                if !is_equal_type(&src_type, &target_type) {
                    errors.extend(vec![err!(
                        src_type,
                        "Type '{} cannot be merged into field of type '{}'.",
                        src_type.to_token_stream(),
                        target_type.to_token_stream()
                    )]);
                } else {
                    let snippet = quote! {
                        #target_field_ident: src.#src_field_ident,
                    };
                    assignments.extend(vec![snippet]);
                }
            }
            // The src is optional and needs to be `Some(T)` to be merged.
            (
                FieldType::Optional {
                    inner: src_type, ..
                },
                FieldType::Normal(_),
            ) => {
                errors.extend(vec![err!(
                    src_type,
                    "Inter-struct cannot 'into' an optional into a non-optional value."
                )]);
            }
            // The target is optional and needs to be wrapped in `Some(T)` to be merged.
            (
                FieldType::Normal(src_type),
                FieldType::Optional {
                    inner: target_type, ..
                },
            ) => {
                if !is_equal_type(&src_type, &target_type) {
                    errors.extend(vec![err!(
                        src_type,
                        "Type '{} cannot be merged into field of type '{}'.",
                        src_type.to_token_stream(),
                        target_type.to_token_stream()
                    )]);
                } else {
                    let snippet = quote! {
                        #target_field_ident: Some(src.#src_field_ident),
                    };
                    assignments.extend(vec![snippet]);
                }
            }
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
                    if !is_equal_type(&inner_src_type, &inner_target_type) {
                        errors.extend(vec![err!(
                            inner_src_type,
                            "Type '{} cannot be merged into field of type '{}'.",
                            inner_src_type.to_token_stream(),
                            inner_target_type.to_token_stream()
                        )]);
                    } else {
                        let snippet = quote! {
                            #target_field_ident: src.#src_field_ident,
                        };
                        assignments.extend(vec![snippet]);
                    }

                    continue;
                }

                // Handling the (src: Option<Option<<T>>, target: Option<T>) case
                if is_equal_type(&inner_src_type, &outer_target_type) {
                    errors.extend(vec![err!(
                        inner_src_type,
                        "Inter-struct cannot 'into' an optional into a non-optional value."
                    )]);
                    continue;
                }

                // Handling the (src: Option<<T>, target: Option<Option<T>)> case
                if !is_equal_type(&outer_src_type, &inner_target_type) {
                    errors.extend(vec![err!(
                        outer_src_type,
                        "Type '{} cannot be merged into field of type '{}'.",
                        outer_src_type.to_token_stream(),
                        inner_target_type.to_token_stream()
                    )]);
                } else {
                    let snippet = quote! {
                        #target_field_ident: Some(src.#src_field_ident),
                    };
                    assignments.extend(vec![snippet]);
                }
            }
            // Skip anything where either of the fields are invalid
            (FieldType::Invalid, _) | (_, FieldType::Invalid) => continue,
        };
    }

    let target_path = &params.target_path;
    let assignment_code = assignments.to_token_stream();
    let error_code = errors.to_token_stream();

    let default_code = if default_impl {
        quote! {
            ..#target_path::default()
        }
    } else {
        quote! {}
    };

    quote! {
        #error_code
        #target_path {
            #assignment_code
            #default_code
        }
    }
}
