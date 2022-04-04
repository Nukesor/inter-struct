use quote::ToTokens;
use syn::Type;

/// A small helper macro, which compares the token streams of two types and enforces their
/// equality. If they aren't equal, a compiler error will be shown.
macro_rules! equal_type_or_err {
    ($src_type:ident, $target_type:ident, $correct_macro:expr) => {
        if !is_equal_type(&$src_type, &$target_type) {
            err!(
                $src_type,
                "Type '{} cannot be merged into field of type '{}'.",
                $src_type.to_token_stream(),
                $target_type.to_token_stream()
            )
        } else {
            $correct_macro
        }
    };
}
pub(crate) use equal_type_or_err;

/// Check whether two given [Type]s are of the same type.
/// If they aren't, an error is added to the src_type and the function returns `false`.
///
/// This check is rather crude, as we simply compare the token streams.
/// However, this is the only way for now, as there are no type infos at this stage.
pub fn is_equal_type(src_type: &Type, target_type: &Type) -> bool {
    if src_type.to_token_stream().to_string() != target_type.to_token_stream().to_string() {
        return false;
    }

    true
}
