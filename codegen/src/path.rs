use std::path::PathBuf;

use proc_macro2::TokenStream;
#[cfg(feature = "debug")]
use quote::ToTokens;
use syn::{Expr, ExprLit, ItemStruct, Lit, Path};

use crate::error::*;

/// Extract the input paths from the macro arguments.
///
/// Both, a single path and an array of paths is supported.
/// I.e.
/// - `merge_struct("crate::some_path::Struct")`
/// - `merge_struct(["crate::some::Struct", "crate::some_other::Struct"])`
pub fn parse_input_paths(args: Expr) -> Result<Vec<Path>, TokenStream> {
    let expr_paren = if let Expr::Paren(expr_paren) = args {
        expr_paren
    } else {
        #[cfg(feature = "debug")]
        println!("Expected group found: {:?}", args.to_token_stream());
        #[cfg(feature = "debug")]
        crate::debug::print_expr_type(args);

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
        Expr::Lit(expr) => {
            #[cfg(feature = "debug")]
            println!("Found path expr: {:?}", expr.to_token_stream());
            lit_to_path(expr).map(|path| vec![path])
        }
        // Handle the caes of an array of strings, containing paths.
        Expr::Array(array) => {
            let mut paths = vec![];
            #[cfg(feature = "debug")]
            println!("Found path expr array: {:?}", array.to_token_stream());
            for expr in array.elems {
                match expr {
                    Expr::Lit(expr) => {
                        #[cfg(feature = "debug")]
                        println!("Path expr in array: {:?}", expr.to_token_stream());
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
        _ => {
            #[cfg(feature = "debug")]
            println!("Found no path expr: {:?}", expr_paren.to_token_stream());
            Err(err!(
                expr_paren,
                "inter_struct's macro parameters should be either a single path {} ",
                "or a vector of paths as str, such as '[\"crate::your::path\"]'."
            ))
        }
    }
}

/// Get the root path of the crate that's currently using this proc macro.
/// This is done via the `CARGO_MANIFEST_DIR` variable, that's always supplied by cargo and
/// represents the directory containing the `Cargo.toml` for the current crate.
pub fn get_root_src_path(span: &ItemStruct) -> Result<PathBuf, TokenStream> {
    match std::env::var("CARGO_MANIFEST_DIR") {
        Err(error) => {
            return Err(err!(
                span,
                "Couldn't read CARGO_MANIFEST_DIR environment variable in InterStruct: {}",
                error
            ));
        }
        Ok(path) => {
            let mut path = PathBuf::from(path);
            if !path.exists() {
                return Err(err!(
                    span,
                    "CARGO_MANIFEST_DIR path doesn't exist in InterStruct: {:?}",
                    path
                ));
            }

            // TODO: We expect the source tree to start in `$CARGO_MANIFEST_DIR/src`.
            // For everything else, we would have to manually parse the Cargo manifest.
            path.push("src");
            if !path.exists() {
                return Err(err!(
                    span,
                    "InterStruct currently expects the source to be located in $CARGO_MANIFEST_DIR/src"
                ));
            }

            Ok(path)
        }
    }
}
