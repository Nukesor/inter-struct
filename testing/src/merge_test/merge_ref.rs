#[cfg(test)]
mod tests {
    use inter_struct::prelude::*;

    use crate::merge_test::*;

    /// Test the normal [StructMerge::merge_ref] function for a struct with the exact same structure.
    #[test]
    fn merge_identical() {
        // The base struct that's going to be merged into.
        let mut base = Base::new();

        // Merge a struct with identical field types.
        let identical = Identical::new();
        base.merge_ref(&identical);
        assert_eq!(base.normal, "identical");
        assert_eq!(base.optional, Some("identical".to_string()));
        assert_eq!(base.ignored, "base");
    }

    /// Test the normal [StructMerge::merge_ref] function for a similar struct, but all fields are
    /// optional.
    #[test]
    fn merge_optional() {
        // The base struct that's going to be merged into.
        let mut base = Base::new();

        // Merge a struct with the same field types, but they're optional.
        let optional = Optional::new();
        base.merge_ref(&optional);
        assert_eq!(base.normal, "optional");
        assert_eq!(base.optional, Some("optional".to_string()));
        assert_eq!(base.ignored, "base");
    }

    /// Test the normal [StructMerge::merge_ref] function for a similar struct, but some fields are
    /// optional and some are identical.
    #[test]
    fn merge_mixed() {
        // The base struct that's going to be merged into.
        let mut base = Base::new();

        // Merge a struct with both, identical and optional fields.
        let mixed = Mixed::new();
        base.merge_ref(&mixed);
        assert_eq!(base.normal, "mixed");
        assert_eq!(base.optional, Some("mixed".to_string()));
        assert_eq!(base.ignored, "base");
    }
}
