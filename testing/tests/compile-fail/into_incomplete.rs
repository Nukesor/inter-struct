/// This should crash, since the generated initializer for IntoStruct will be incomplete.
use inter_struct::InterStruct;

#[derive(InterStruct)]
#[into("crate::IntoStruct")]
pub struct FromStruct {
    pub normal: String,
}

/// A struct with less, but otherwise identical fields.
pub struct IntoStruct {
    pub normal: String,
    pub optional: Option<String>,
}

fn main() {}
