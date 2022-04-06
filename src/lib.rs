pub use inter_struct_codegen::*;

/// Docs and traits for struct merging.
pub mod merge;

/// Imports all modules to get you started.
pub mod prelude {
    pub use super::merge::*;
    pub use inter_struct_codegen::*;
}
