use inter_struct::prelude::*;

/// This should crash, since the generated initializer for IntoStruct will be incomplete.
#[derive(StructInto)]
#[struct_into("crate::IntoStruct")]
pub struct FromStruct {
    pub normal: String,
}

pub struct IntoStruct {
    pub normal: String,
    pub optional: Option<String>,
}

fn main() {}
