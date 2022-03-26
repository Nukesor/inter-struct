/// Helper macro, which attaches an error to a given span.
macro_rules! err {
    ($span:expr, $($text:expr),*) => {
        {
            #[allow(unused_imports)]
            use syn::spanned::Spanned;
            let message = format!($($text,)*);
            let span = $span.span();
            quote::quote_spanned!( span => compile_error!(#message); )
        }
    }
}
pub(crate) use err;

// Uncomment this as soon as proc_macro_diagnostic land in stable.
//
//#![feature(proc_macro_diagnostic)]
///// Helper macro, which attaches an error to a given span.
//macro_rules! err {
//    ($span:ident, $($text:expr),*) => {
//        $span.span()
//            .unwrap()
//            .error(format!($($text,)*))
//            .emit();
//    }
//}

/// Helper macro, which takes a result.
/// Ok(T) => simply return the T
/// Err(err) => Emits an compiler error on the given span with the provided error message.
///             Also returns early with `None`.
///             `None` is used throughout this crate as a gracefull failure.
///             That way all code that can be created is being generated and the user sees all
///             errors without the macro code panicking.
macro_rules! ok_or_err_return {
    ($expr:expr, $span:ident, $($text:expr),*) => {
        match $expr {
            Ok(result) => result,
            Err(error) =>  {
                return Err(err!($span, $($text,)* error));
            }
        }
    }
}
pub(crate) use ok_or_err_return;
