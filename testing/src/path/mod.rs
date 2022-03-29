use inter_struct::prelude::*;

pub mod file;

pub struct InModFile {
    pub field: String,
}

/// A simple test struct, which
#[derive(InterStruct)]
#[into(["crate::RootLevelFile", "crate::path::InModFile",  "crate::path::file::InNormalFile"])]
pub struct TestStruct {
    pub field: String,
}

pub mod submod {
    pub struct SubModInModFile {
        pub field: String,
    }
}
