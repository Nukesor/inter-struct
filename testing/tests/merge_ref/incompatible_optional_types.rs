use inter_struct::prelude::*;

/// This should produce a compile errors, as the field types for `other` are different.
#[derive(InterStruct, Clone)]
#[merge_ref("crate::MergeStruct")]
pub struct FromStruct {
    pub normal: Option<i32>,
    pub optional: Option<Option<i32>>,
    pub optional_optional: Option<Option<String>>,
}

pub struct MergeStruct {
    pub normal: String,
    pub optional: Option<String>,
    pub optional_optional: Option<Option<String>>,
}

fn main() {}
