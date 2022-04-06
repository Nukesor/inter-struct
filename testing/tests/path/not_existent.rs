use inter_struct::prelude::*;

/// This shouldn't compile, as the path shows to an invalid location.
#[derive(StructInto)]
#[struct_into("crate::some_path::IntoStruct")]
pub struct FromStruct {
    pub normal: String,
}

pub struct IntoStruct {
    pub normal: String,
    pub optional: Option<String>,
}

fn main() {}
