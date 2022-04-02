pub struct IntoStruct {
    pub normal: String,
    pub optional: Option<String>,
}

pub struct MergeStruct {
    pub normal: String,
    pub optional: Option<String>,
    pub optional_optional: Option<Option<String>>,
}
