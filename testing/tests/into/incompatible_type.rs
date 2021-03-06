use inter_struct::prelude::*;

/// This should crash, since the types for the IntoStruct are incompatible.
#[derive(StructInto)]
#[struct_into("crate::IntoStruct")]
pub struct FromStruct {
    pub normal: i32,
    pub optional: Option<i32>,
}

pub struct IntoStruct {
    pub normal: String,
    pub optional: Option<String>,
}

fn main() {}
