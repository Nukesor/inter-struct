use inter_struct::InterStruct;

/// This shouldn't compile, as the path shows to an invalid location.
#[derive(InterStruct)]
#[into("crate::some_path::IntoStruct")]
pub struct FromStruct {
    pub normal: String,
}

pub struct IntoStruct {
    pub normal: String,
    pub optional: Option<String>,
}

fn main() {}
