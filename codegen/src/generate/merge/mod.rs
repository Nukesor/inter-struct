use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemStruct};

use super::{inter_struct_base, Mode};
use crate::helper::get_root_src_path;
use crate::parse;

pub mod borrowed;
pub mod owned;

/// The actual logic for the struct_merge derive macro.
pub fn struct_merge_inner(struct_ast: TokenStream) -> TokenStream {
    // Parse the main macro input as a struct.
    // We work on a clone of the struct ast.
    // That way we don't have to parse it again, when we return it lateron.
    let src_struct = parse_macro_input!(struct_ast as ItemStruct);

    // Check if we can find the src root path of this crate.
    // Return early if it doesn't exist.
    let src_root_path = match get_root_src_path(&src_struct) {
        Ok(path) => path,
        Err(err) => return TokenStream::from(err),
    };

    // Check if we can find the src root path of this crate.
    // Return early if it doesn't exist.
    let attribute = match parse::attribute(&src_struct, "StructMerge", "struct_merge") {
        Ok(attribute) => attribute,
        Err(err) => return TokenStream::from(err),
    };

    let parsed_args = match attribute.parse_args() {
        Ok(parsed_args) => parsed_args,
        Err(err) => return err.into_compile_error().into(),
    };

    let impls = inter_struct_base(&src_root_path, &src_struct, parsed_args, Mode::Merge);

    // Merge all generated pieces of the code with the original unaltered struct.
    let mut tokens = TokenStream::new();
    tokens.extend(impls.into_iter().map(TokenStream::from));

    #[cfg(feature = "debug")]
    println!("StructMerge impl: {}", tokens.to_string());

    tokens
}

/// The actual logic for the struct_merge_ref derive macro.
pub fn struct_merge_ref_inner(struct_ast: TokenStream) -> TokenStream {
    // Parse the main macro input as a struct.
    // We work on a clone of the struct ast.
    // That way we don't have to parse it again, when we return it lateron.
    let src_struct = parse_macro_input!(struct_ast as ItemStruct);

    // Check if we can find the src root path of this crate.
    // Return early if it doesn't exist.
    let src_root_path = match get_root_src_path(&src_struct) {
        Ok(path) => path,
        Err(err) => return TokenStream::from(err),
    };

    // Check if we can find the src root path of this crate.
    // Return early if it doesn't exist.
    let attribute = match parse::attribute(&src_struct, "StructMergeRef", "struct_merge_ref") {
        Ok(attribute) => attribute,
        Err(err) => return TokenStream::from(err),
    };

    let parsed_args = match attribute.parse_args() {
        Ok(parsed_args) => parsed_args,
        Err(err) => return err.into_compile_error().into(),
    };

    let impls = inter_struct_base(&src_root_path, &src_struct, parsed_args, Mode::MergeRef);

    // Merge all generated pieces of the code with the original unaltered struct.
    let mut tokens = TokenStream::new();
    tokens.extend(impls.into_iter().map(TokenStream::from));

    #[cfg(feature = "debug")]
    println!("StructMergeRef impl: {}", tokens.to_string());

    tokens
}
