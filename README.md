# Inter-Struct

[![GitHub Actions Workflow](https://github.com/nukesor/inter-struct/workflows/Test%20build/badge.svg)](https://github.com/Nukesor/inter-struct/actions)
[![Crates.io](https://img.shields.io/crates/v/inter-struct)](https://crates.io/crates/inter-struct)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Downloads](https://img.shields.io/github/downloads/nukesor/inter-struct/total.svg)](https://github.com/nukesor/inter-struct/releases)

Inter-struct provides various derive macros to implement traits between two structs.

This is useful to, for instance, automatically generate traits such as `Into` or `PartialEq` between two similar structs.

Please read the **known caveats** section before using this crate!
It's not trivial to implement code for two different structs in a codebase.

Also note that this crate is in an early development phase.
The crate is already properly tested, but bugs might still be there and the API might change drastically.

## Features:

- Merge - Merge a struct into another, while consuming itself.
- MergeRef - Merge a struct into another by reference. The struct must implement `Clone`.
- Into - A standard `From/Into` impl between two structs.
- IntoDefault - `From/Into`, but use `Default` on the target for unknown fields.

## Merge

```rust,ignore
/// Merge another struct into Self whilst consuming it.
/// 
/// The other trait is named `StructMergeRef` and merges other structs by reference.
pub trait StructMerge<Src> {
    /// Merge the given struct into self.
    fn merge(&mut self, src: Src);
}
```

This following code is an example on how to use the `StructMerge` derive macro for implementing the `StructMerge` trait between two structs.

```rust,ignore
use inter_struct::prelude::*;

/// The target struct we'll merge into.
pub struct Target {
    pub normal: String,
    pub optional: String,
    /// This field won't be touched as the macro cannot find a
    /// respective `ignored` field in the `Source` struct.
    pub ignored: String,
}

/// A struct with both an identical and an optional field type.
/// Note that the path to `Target` must always be fully qualifying.
#[derive(StructMerge)]
#[merge("crate::Target")]
pub struct Source {
    pub normal: String,
    pub optional: Option<String>,
}

fn main() {
    let mut target = Target {
        normal: "target".to_string(),
        optional: "target".to_string(),
        ignored: "target".to_string(),
    };

    let source = Source {
        /// Has the same type as Target::normal
        normal: "source".to_string(),
        /// Wraps Target::optional in an Option
        optional: Some("source".to_string()),
    };

    // Merge the `Source` struct into target.
    target.merge(source);
    // You can also call this:
    // source.merge_into(target);
    assert_eq!(target.normal, "source".to_string());
    assert_eq!(target.optional, Some("source".to_string()));
    assert_eq!(target.ignored, "target".to_string());
}
```


## Into


This following code is an example on how to use the `StructInto` derive macro for implementing `Into` between two structs.

```rust,ignore
use inter_struct::prelude::*;

/// The target struct we'll convert our `Source` struct into.
pub struct Target {
    pub normal: String,
    pub optional: String,
}

#[derive(StructInto)]
// Note that the path to `Target` must always be fully qualifying.
#[struct_into("crate::Target")]
pub struct Source {
    pub normal: String,
    pub optional: Option<String>,
    /// This field doesn't exist in the target, hence it'll be ignored.
    pub ignored: String,
}

fn main() {
    let source = Source {
        /// Has the same type as Target::normal
        normal: "source".to_string(),
        /// Wraps Target::optional in an Option
        optional: Some("source".to_string()),
        ignored: "source".to_string(),
    };

    // Convert the `Source` struct into `Target`.
    let target: Target = source.into();
    assert_eq!(target.normal, "source".to_string());
    assert_eq!(target.optional, Some("source".to_string()));
}
```

## Known caveats

Inter-struct is designed to work in this environment:

- In the scope of a single crate. Cross-crate usage won't work.
- In the main `src` folder of the crate. Integration tests and examples aren't supported.

The main problems in this crate come from the fact that there's no official way to resolve modules or types in the the procedural macro stage.

Due to this limitation, inter-struct isn't capable of ensuring the equality of two types.
As a result, it might create false negative compile errors, even though the types might be compatible.
This might happen if, for instance, types are obscured via an alias or if a type can be automatically dereferenced into another type.

However, as we're creating safe and valid Rust code, the compiler will thrown an error if any type problems arise.


#### Not yet solved problems

These are problems that can probably be solved but they're non-trivial.

- [ ] Struct is located in integration tests.
- [ ] Struct in (potentially nested or alternating) `mod {}` block in file.
- [ ] The source root dir isn't `src`.
      We would have to check the environment and possibly parse the `Cargo.toml`.

#### Unsolvable problems

These are problems that are either impossible to solve or very infeasible.
For instance, something infeasible would be to parse all files for a full type resolution of a given crate.
That would be a job for the compiler in a later stage.

- Structs that are altered or generated by other macros.
- Type comparison and type resolution. E.g. `type test = Option<String>` won't be detected as optional.
    The current type checks are literal comparisons of the type tokens.
    Also, the type alias `type AlsoString = String;` won't be detected as a `String`;
- Non-public structs. I.e. structs that aren't fully internally visible.
    This will lead to an compiler-error but isn't cought while running this macro.
