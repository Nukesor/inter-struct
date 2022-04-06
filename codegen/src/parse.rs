use proc_macro2::TokenStream;
#[cfg(feature = "debug")]
use quote::ToTokens;
use syn::{Attribute, Expr, ExprLit, ItemStruct, Lit, Path};

use crate::error::err;

/// Parse the main attribute of the derive macro.
///
/// It basically parses all attributes and returns the attribute that matches `name`.
/// If there's no matching attribute, an appropriate compiler error message is thrown.
pub fn attribute(
    src_struct: &ItemStruct,
    derive_name: &str,
    name: &str,
) -> Result<Attribute, TokenStream> {
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
        src_struct.ident,
        "{} requires the '{}' attribute.",
        derive_name,
        name
    ))
}

/// Extract the input paths from the macro arguments.
///
/// Both, a single path and an array of paths is supported.
/// I.e.
/// - `merge_struct("crate::some_path::Struct")`
/// - `merge_struct(["crate::some::Struct", "crate::some_other::Struct"])`
pub fn input_paths(args: Expr) -> Result<Vec<Path>, TokenStream> {
    let expr_paren = if let Expr::Paren(expr_paren) = args {
        expr_paren
    } else {
        #[cfg(feature = "debug")]
        println!("Expected group found: {:?}", args.to_token_stream());
        #[cfg(feature = "debug")]
        crate::debug::print_expr_type(args.clone());

        return Err(err!(args, "Encountered unknown error while parsing args."));
    };

    fn lit_to_path(expr: ExprLit) -> Result<Path, TokenStream> {
        match expr.lit {
            // Make sure we got a literal string.
            Lit::Str(lit_str) => match lit_str.parse_with(syn::Path::parse_mod_style) {
                Err(_) => Err(err!(
                    lit_str,
                    "Only paths are allowed in inter_struct's attributes."
                )),
                Ok(path) => Ok(path),
            },
            _ => Err(err!(
                expr,
                "Only paths are allowed in inter_struct's attribute."
            )),
        }
    }

    match *expr_paren.expr {
        // Handle the first case of a single string containing a path.
        Expr::Lit(expr) => lit_to_path(expr).map(|path| vec![path]),
        // Handle the caes of an array of strings, containing paths.
        Expr::Array(array) => {
            let mut paths = vec![];
            for expr in array.elems {
                match expr {
                    Expr::Lit(expr) => {
                        let path = lit_to_path(expr)?;
                        paths.push(path);
                    }
                    _ => {
                        err!(expr, "Only paths are allowed in inter_struct's attribute.");
                    }
                }
            }
            Ok(paths)
        }
        _ => Err(err!(
            expr_paren,
            "inter_struct's macro parameters should be either a single path {} ",
            "or a vector of paths as str, such as '[\"crate::your::path\"]'."
        )),
    }
}
