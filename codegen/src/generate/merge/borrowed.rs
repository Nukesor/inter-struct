use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Field;

use crate::error::*;
use crate::generate::field::*;
use crate::generate::types::*;
use crate::Parameters;

/// Generate the implementation of [inter_struct::merge::StructMergeRef] for given structs.
pub(crate) fn impl_borrowed(params: &Parameters, fields: Vec<(Field, Field)>) -> TokenStream {
    let mut functions_tokens = TokenStream::new();

    // Add `merge_ref` impl.
    let stream = merge_ref(params, fields);
    functions_tokens.extend(vec![stream]);

    // Surround functions with `impl` block.
    let src_ident = &params.src_struct.ident;
    let target_path = &params.target_path;
    quote! {
        impl inter_struct::merge::StructMergeIntoRef<#target_path> for #src_ident {
            #functions_tokens
        }
    }
}

/// Generate the [inter_struct::merge::StructMergeRef::merge_ref] function for given structs.
///
/// All fields must implement `Clone`.
fn merge_ref(params: &Parameters, fields: Vec<(Field, Field)>) -> TokenStream {
    let mut merge_code = TokenStream::new();
    for (src_field, dest_field) in fields {
        let src_field_ident = src_field.ident;
        let dest_field_ident = dest_field.ident;

        // Find out, whether the fields are optional or not.
        let src_field_type = match determine_field_type(src_field.ty) {
            Ok(field) => field,
            Err(err) => {
                merge_code.extend(vec![err]);
                continue;
            }
        };
        let target_field_type = match determine_field_type(dest_field.ty) {
            Ok(field) => field,
            Err(err) => {
                merge_code.extend(vec![err]);
                continue;
            }
        };

        let snippet = match (src_field_type, target_field_type) {
            // Both fields have the same type
            (FieldType::Normal(src_type), FieldType::Normal(target_type)) => {
                equal_type_or_err!(
                    src_type,
                    target_type,
                    "",
                    quote! {
                        target.#dest_field_ident = self.#src_field_ident.clone();
                    }
                )
            }
            // The src is optional and needs to be `Some(T)` to be merged.
            (
                FieldType::Optional {
                    inner: src_type, ..
                },
                FieldType::Normal(target_type),
            ) => {
                equal_type_or_err!(
                    src_type,
                    target_type,
                    "Inner ",
                    quote! {
                        if let Some(value) = self.#src_field_ident.as_ref() {
                            target.#dest_field_ident = value.clone();
                        }
                    }
                )
            }
            // The target is optional and needs to be wrapped in `Some(T)` to be merged.
            (
                FieldType::Normal(src_type),
                FieldType::Optional {
                    inner: target_type, ..
                },
            ) => {
                equal_type_or_err!(
                    src_type,
                    target_type,
                    "",
                    quote! {
                        self.#dest_field_ident = Some(src.#src_field_ident.clone());
                    }
                )
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
                    quote! {
                        target.#dest_field_ident = self.#src_field_ident.clone();
                    }
                // Handling the (Option<Option<<T>>, Option<T>) case
                } else if is_equal_type(&inner_src_type, &outer_target_type) {
                    quote! {
                        if let Some(value) = self.#src_field_ident.as_ref() {
                            target.#dest_field_ident = value.clone();
                        }
                    }
                // Handling the (Option<<T>, Option<Option<T>)> case
                } else {
                    equal_type_or_err!(
                        outer_src_type,
                        inner_target_type,
                        "",
                        quote! {
                            target.#dest_field_ident = Some(self.#src_field_ident.clone());
                        }
                    )
                }
            }
            // Skip anything where either of the fields are invalid
            (FieldType::Invalid, _) | (_, FieldType::Invalid) => continue,
        };

        merge_code.extend(vec![snippet]);
    }

    let merge_code = merge_code.to_token_stream();

    let target_path = &params.target_path;
    quote! {
        fn merge_into_ref(&self, target: &mut #target_path) {
            #merge_code
        }
    }
}
