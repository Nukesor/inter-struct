use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Field;

use crate::error::*;
use crate::generate::field::*;
use crate::generate::types::*;
use crate::Parameters;

/// Generate the implementation of [inter_struct::merge::StructMerge] for given structs.
pub(crate) fn impl_owned(params: &Parameters, fields: Vec<(Field, Field)>) -> TokenStream {
    let mut functions_tokens = TokenStream::new();

    // Add `merge` impl.
    let stream = merge(params, fields);
    functions_tokens.extend(vec![stream]);

    // Surround functions with `impl` block.
    let src_ident = &params.src_struct.ident;
    let target_path = &params.target_path;
    quote! {
        impl inter_struct::merge::StructMergeInto<#target_path> for #src_ident {
            #functions_tokens
        }
    }
}

/// Generate the [inter_struct::merge::StructMerge::merge] function for the given structs.
fn merge(params: &Parameters, fields: Vec<(Field, Field)>) -> TokenStream {
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
        let dest_field_type = match determine_field_type(dest_field.ty) {
            Ok(field) => field,
            Err(err) => {
                merge_code.extend(vec![err]);
                continue;
            }
        };

        let snippet = match (src_field_type, dest_field_type) {
            // Both fields have the same type
            (FieldType::Normal(_), FieldType::Normal(_)) => {
                quote! {
                    dest.#dest_field_ident = self.#src_field_ident;
                }
            }
            // The src is optional and needs to be `Some(T)` to be merged.
            (FieldType::Optional { .. }, FieldType::Normal(_)) => {
                quote! {
                    if let Some(value) = self.#src_field_ident {
                        dest.#dest_field_ident = value;
                    }
                }
            }
            // The dest is optional and needs to be wrapped in `Some(T)` to be merged.
            (FieldType::Normal(_), FieldType::Optional { .. }) => {
                quote! {
                    dest.#dest_field_ident = Some(self.#src_field_ident);
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
                    inner: inner_dest_type,
                    outer: outer_dest_type,
                },
            ) => {
                // Handling the (Option<T>, Option<T>) case
                if is_equal_type(&inner_src_type, &inner_dest_type) {
                    quote! {
                        dest.#dest_field_ident = self.#src_field_ident;
                    }
                // Handling the (Option<Option<<T>>, Option<T>) case
                } else if is_equal_type(&inner_src_type, &outer_dest_type) {
                    quote! {
                        if let Some(value) = self.#src_field_ident {
                            dest.#dest_field_ident = value;
                        }
                    }
                // Handling the (Option<<T>, Option<Option<T>)> case
                } else {
                    equal_type_or_err!(
                        outer_src_type,
                        inner_dest_type,
                        "",
                        quote! {
                            dest.#dest_field_ident = Some(self.#src_field_ident);
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
        fn merge_into(self, dest: &mut #target_path) {
            #merge_code
        }
    }
}
