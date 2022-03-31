pub mod into_test;
pub mod merge_test;
pub mod path;

pub struct RootLevelFile {
    pub field: String,
}

/// A struct with less, but otherwise identical fields.
pub struct IntoStruct {
    pub normal: String,
    pub optional: Option<String>,
}
