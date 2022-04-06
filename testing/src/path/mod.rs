use inter_struct::prelude::*;

pub mod file;

pub struct InModFile {
    pub field: String,
}

/// A simple test struct, which ensures that various path resolutions work as expected
#[derive(StructInto)]
#[struct_into(["crate::RootLevelFile", "crate::path::InModFile",  "crate::path::file::InNormalFile"])]
pub struct TestStruct {
    pub field: String,
}

pub mod submod {
    pub struct SubModInModFile {
        pub field: String,
    }
}
