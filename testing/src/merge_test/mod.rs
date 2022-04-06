#![allow(clippy::new_without_default)]

use inter_struct::prelude::*;
mod merge;
mod merge_ref;

pub struct Base {
    pub normal: String,
    pub optional: Option<String>,
    pub ignored: String,
}

impl Base {
    pub fn new() -> Self {
        Base {
            normal: "base".to_string(),
            optional: Some("base".to_string()),
            ignored: "base".to_string(),
        }
    }
}

/// A struct with identical field types.
/// Note that the path to `Base` is always a fully qualifying path.
#[derive(StructMerge, StructMergeRef, Clone)]
#[struct_merge("crate::merge_test::Base")]
#[struct_merge_ref("crate::merge_test::Base")]
pub struct Identical {
    pub normal: String,
    pub optional: Option<String>,
}

impl Identical {
    pub fn new() -> Self {
        Identical {
            normal: "identical".to_string(),
            optional: Some("identical".to_string()),
        }
    }
}

/// A struct with the same field types as [Base], but the're optional.
/// Note that the path to `Base` is always a fully qualifying path.
#[derive(StructMerge, StructMergeRef, Clone)]
#[struct_merge("crate::merge_test::Base")]
#[struct_merge_ref("crate::merge_test::Base")]
pub struct Optional {
    pub normal: Option<String>,
    pub optional: Option<Option<String>>,
}

impl Optional {
    pub fn new() -> Self {
        Optional {
            normal: Some("optional".to_string()),
            optional: Some(Some("optional".to_string())),
        }
    }
}

/// A struct with both, identical and optional field types.
/// Note that the path to `Base` is always a fully qualifying path.
#[derive(StructMerge, StructMergeRef, Clone)]
#[struct_merge("crate::merge_test::Base")]
#[struct_merge_ref("crate::merge_test::Base")]
pub struct Mixed {
    pub normal: String,
    pub optional: Option<Option<String>>,
}

impl Mixed {
    pub fn new() -> Self {
        Mixed {
            normal: "mixed".to_string(),
            optional: Some(Some("mixed".to_string())),
        }
    }
}
