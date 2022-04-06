use std::path::PathBuf;

use proc_macro2::TokenStream;
use syn::ItemStruct;

use crate::error::*;

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
