use proc_macro2::TokenStream;
use syn::spanned::Spanned;
use syn::{Fields, GenericArgument, PathArguments, Type};

use crate::{Mode, Parameters};

pub mod merge;

/// Internal representation of parsed types
///
/// We either expect fields to have a generic type `T` or `Option<T>`.
/// Allow dead code, since this is what we're going to use as soon as proc macro hygiene has
/// improved.
#[allow(clippy::large_enum_variant)]
#[allow(dead_code)]
enum FieldType {
    Normal(Type),
    Optional { inner: Type, outer: Type },
    Invalid,
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
    }
}

/// This function takes any [Type] and determines, whether it's an `Option<T>` or just a `T`.
///
/// This detected variant is represented via the [FieldType] enum.
/// Invalid or unsupported types return the `FieldType::Invalid` variant.
///
/// Known limitations:
///
/// This doesn't work with type aliases. We literally check the tokens for `Option<...>`.
/// If there's an optional type that doesn't look like this, we won't detect it.
fn determine_field_type(ty: Type) -> Result<FieldType, TokenStream> {
    match ty.clone() {
        Type::Path(type_path) => {
            // The path is relative to `Self` and thereby non-optional
            if type_path.qself.is_some() {
                return Ok(FieldType::Normal(ty));
            }

            let path = type_path.path;

            // `Option<T>` shouldn't have a leading colon or multiple segments.
            if path.leading_colon.is_some() || path.segments.len() > 1 {
                return Ok(FieldType::Normal(ty));
            }

            // The path should have at least one segment.
            let segment = if let Some(segment) = path.segments.iter().next() {
                segment
            } else {
                return Ok(FieldType::Normal(ty));
            };

            // The segment isn't an option.
            if segment.ident != "Option" {
                return Ok(FieldType::Normal(ty));
            }

            // Get the angle brackets
            let generic_arg = match &segment.arguments {
                PathArguments::AngleBracketed(params) => {
                    if let Some(arg) = params.args.iter().next() {
                        arg
                    } else {
                        return Err(err!(ty, "Option doesn't have a type parameter.."));
                    }
                }
                _ => {
                    return Err(err!(
                        ty,
                        "Unknown path arguments behind Option. Please report this."
                    ))
                }
            };

            // This argument must be a type:
            match generic_arg {
                GenericArgument::Type(inner_type) => Ok(FieldType::Optional {
                    inner: inner_type.clone(),
                    outer: ty,
                }),
                _ => Err(err!(ty, "Option path argument isn't a type.")),
            }
        }
        _ => Err(err!(
            ty,
            "Found a non-path type. This isn't supported in inter-struct yet."
        )),
    }
}
