#![allow(clippy::new_without_default)]

use inter_struct::prelude::*;

mod into;
mod into_default;

#[derive(StructInto, StructIntoDefault)]
#[struct_into("crate::into_test::IntoStruct")]
#[struct_into_default("crate::into_test::IntoDefaultStruct")]
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

/// A struct with a few additional fields that should be populated by their [Default] values.
#[derive(Default)]
pub struct IntoDefaultStruct {
    pub normal: String,
    pub optional: Option<String>,
    pub normal_additional: String,
    pub optional_additional: Option<String>,
}
