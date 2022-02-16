pub mod file;

pub struct InModFile {
    pub field: &'static str,
}

pub mod submod {
    pub struct SubModInModFile {
        pub field: &'static str,
    }
}
