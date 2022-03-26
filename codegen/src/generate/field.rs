use proc_macro2::TokenStream;
use syn::{GenericArgument, PathArguments, Type};

use crate::error::*;

/// Internal representation of parsed types
///
/// We either expect fields to have a generic type `T` or `Option<T>`.
/// Allow dead code, since this is what we're going to use as soon as proc macro hygiene has
/// improved.
#[allow(clippy::large_enum_variant)]
#[allow(dead_code)]
pub enum FieldType {
    Normal(Type),
    Optional { inner: Type, outer: Type },
    Invalid,
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
pub fn determine_field_type(ty: Type) -> Result<FieldType, TokenStream> {
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
