use std::path::PathBuf;

use proc_macro2::TokenStream;
use syn::{spanned::Spanned, Ident, Item, ItemStruct, Path, Token};

/// This function takes a path to a struct and returns the AST of that struct.
///
/// There is no easy way to do module resolution during this stage of the compilation.
pub fn get_struct_from_path(root_path: PathBuf, path: Path) -> Result<ItemStruct, TokenStream> {
    // Start searching for files from the project root.
    let path_span = path.span();

    let mut segments = path.segments.into_iter().peekable();
    // Make sure the root of the path is the current crate.
    let first = segments.next().unwrap();
    let crate_token = Token![crate](first.span());
    let crate_ident = Ident::from(crate_token);
    if first.ident != crate_ident {
        return Err(err!(
            first,
            "inter_struct only supports paths in the current 'crate::' space for now."
        ));
    }

    // In here we try to find the actual file that contains the specified struct.
    // This can be a little tricky, as Rust allows various ways of defining new modules.
    let mut file_path = root_path;
    let target_struct_name = loop {
        // We know that the next value exists.
        // If no further value exists, we break and exit early.
        let segment = segments.next().unwrap();

        // The last identifier is the the name of the struct.
        // Break, so it doen't get added to the path.
        if segments.peek().is_none() {
            break segment.ident;
        }

        // Push the next identifier to the path.
        file_path.push(segment.ident.to_string());

        // Check if we find a folder for that module.
        // If we do, we found our next module.
        if file_path.exists() {
            continue;
        }

        // In case we couldn't find a folder, try a Rust file.
        // Set the extension for rust source code files.
        file_path.set_extension("rs");
        if file_path.exists() {
            continue;
        }

        // TODO: This breaks if there are non-file modules.
        // A much better and more dynamic module resolution is needed for that to work.
    };

    // If the last module was a directory, we have to access it's `mod.rs` file.
    if file_path.is_dir() {
        file_path.push("mod.rs");
    }

    // Read and parse the file.
    let file_content = ok_or_err_return!(
        std::fs::read_to_string(&file_path),
        path_span,
        "Failed to open file: {:?}"
    );

    let file_ast = ok_or_err_return!(
        syn::parse_file(&file_content),
        path_span,
        "Failed to parse file: {:?}"
    );

    for item in file_ast.items.into_iter() {
        if let Item::Struct(item_struct) = item {
            if item_struct.ident == target_struct_name {
                return Ok(item_struct);
            }
        }
    }

    Err(err!(
        path_span,
        "Didn't find struct {} in file {:?}",
        target_struct_name,
        &file_path
    ))
}
