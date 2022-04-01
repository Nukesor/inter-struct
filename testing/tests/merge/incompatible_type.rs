use inter_struct::prelude::*;

/// This should produce a compile errors, as the field types for `other` are different.
#[derive(InterStruct)]
#[merge("crate::MergeStruct")]
pub struct FromStruct {
    pub normal: String,
    pub optional: i32,
}

pub struct MergeStruct {
    pub normal: String,
    pub optional: Option<String>,
}

fn main() {}
