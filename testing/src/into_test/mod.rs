#![allow(clippy::new_without_default)]

use inter_struct::InterStruct;

mod into;

#[derive(InterStruct)]
#[into("crate::into_test::IntoStruct")]
pub struct FromStruct {
    pub normal: String,
    pub optional: Option<String>,
    pub ignored_field: String,
    pub another_ignored_field: Option<String>,
}

impl FromStruct {
    pub fn new() -> Self {
        FromStruct {
            normal: "from".to_string(),
            optional: Some("from".to_string()),
            ignored_field: "from".to_string(),
            another_ignored_field: Some("from".to_string()),
        }
    }
}

/// A struct with less, but otherwise identical fields.
pub struct IntoStruct {
    pub normal: String,
    pub optional: Option<String>,
}
