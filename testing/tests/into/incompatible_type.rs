use inter_struct::InterStruct;

/// This should crash, since the types for the IntoStruct are incompatible.
#[derive(InterStruct)]
#[into("crate::IntoStruct")]
pub struct FromStruct {
    pub normal: i32,
    pub optional: Option<i32>,
}

pub struct IntoStruct {
    pub normal: String,
    pub optional: Option<String>,
}

fn main() {}
