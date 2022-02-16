#[cfg(test)]
mod tests {
    use inter_struct::prelude::*;

    use crate::merge_test::*;

    /// Test the normal [StructMerge::merge] function for a struct with the exact same structure.
    #[test]
    fn merge_identical() {
        // The base struct that's going to be merged into.
        let mut base = Base::new();

        // Merge a struct with identical field types.
        let identical = Identical::new();
        base.merge(identical);
        assert_eq!(base.normal, "identical");
        assert_eq!(base.optional, Some("identical".to_string()));
        assert_eq!(base.ignored, "base");
    }

    /// Test the normal [StructMerge::merge] function for a similar struct, but all fields are
    /// optional.
    #[test]
    fn merge_optional() {
        // The base struct that's going to be merged into.
        let mut base = Base::new();

        // Merge a struct with the same field types, but they're optional.
        let optional = Optional::new();
        base.merge(optional);
        assert_eq!(base.normal, "optional");
        assert_eq!(base.optional, Some("optional".to_string()));
        assert_eq!(base.ignored, "base");
    }

    /// Test the normal [StructMerge::merge] function for a similar struct, but some fields are
    /// optional and some are identical.
    #[test]
    fn merge_mixed() {
        // The base struct that's going to be merged into.
        let mut base = Base::new();

        // Merge a struct with both, identical and optional fields.
        let mixed = Mixed::new();
        base.merge(mixed);
        assert_eq!(base.normal, "mixed");
        assert_eq!(base.optional, Some("mixed".to_string()));
        assert_eq!(base.ignored, "base");
    }

    /// Test the normal [StructMerge::merge_soft] function for a struct with the exact same structure.
    #[test]
    fn merge_soft_identical() {
        // The base struct that's going to be merged into.
        let mut base = Base::new();

        // A struct with identical field types.
        let identical = Identical::new();

        // Only optional fields will be merged in soft-mode.
        // `base.optional` will also not change, as it's already `Some`.
        base.merge_soft(identical);
        assert_eq!(base.normal, "base");
        assert_eq!(base.optional, Some("base".to_string()));
        assert_eq!(base.ignored, "base");
    }

    /// Test the normal [StructMerge::merge_soft] function for a similar struct, but all fields are
    /// optional.
    #[test]
    fn merge_soft_optional() {
        // The base struct that's going to be merged into.
        let mut base = Base::new();
        // Reset to None, so we can observe a merge.
        base.optional = None;

        // A struct with the same field types, but they're optional.
        let optional = Optional::new();

        base.merge_soft(optional);
        assert_eq!(base.normal, "base");
        assert_eq!(base.optional, Some("optional".to_string()));
        assert_eq!(base.ignored, "base");
    }

    /// Test the normal [StructMerge::merge_soft] function for a similar struct, but some fields are
    /// optional and some are identical.
    #[test]
    fn merge_soft_mixed() {
        // The base struct that's going to be merged into.
        let mut base = Base::new();
        // Reset to None, so we can observe a merge.
        base.optional = None;

        // A struct with both, identical and optional fields.
        let mixed = Mixed::new();

        base.merge_soft(mixed);
        assert_eq!(base.normal, "base");
        assert_eq!(base.optional, Some("mixed".to_string()));
        assert_eq!(base.ignored, "base");
    }
}
