use inter_struct::prelude::*;

/// Ensure that error messages for each missing attribute are shown
#[derive(StructInto, StructMerge, StructMergeRef, StructIntoDefault)]
pub struct FromStruct {
    pub normal: String,
}

pub struct IntoStruct {
    pub normal: String,
    pub optional: Option<String>,
}

fn main() {}
