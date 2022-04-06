#[cfg(test)]
mod tests {
    use crate::into_test::*;

    /// Test the implementation of [std::convert::Into] generated by inter-struct.
    #[test]
    fn test_into_with_default() {
        // The base struct that's going to be merged into.
        let from = FromStruct::new();

        let into = IntoDefaultStruct::from(from);
        assert_eq!(into.normal, "from");
        assert_eq!(into.optional, Some("from".to_string()));

        assert_eq!(into.normal_additional, "");
        assert_eq!(into.optional_additional, None);
    }

    /// Test the implementation of [std::convert::Into] generated by inter-struct.
    #[test]
    fn test_into_non_optional_with_default() {
        // The base struct that's going to be merged into.
        let from = FromStructNonOptional::new();

        let into = IntoDefaultStruct::from(from);
        assert_eq!(into.normal, "from_non_optional");
        assert_eq!(into.optional, Some("from_non_optional".to_string()));

        assert_eq!(into.normal_additional, "");
        assert_eq!(into.optional_additional, None);
    }
}
