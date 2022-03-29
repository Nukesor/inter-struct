# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 30-12-2021

This is the first MVP release of the `inter-struct` library.

It's purpose is to implement traits between various structs.

### Added

#### Traits:

- `StructMerge` trait which implements functions to merge a given struct into `Self`.
- `StructMergeInto` trait.
    The counterpart of `StructMerge` which merges `Self` into a target struct.
    `StructMerge` is automatically implemented.
- `StructMergeRef` trait which implements functions to merge a reference of given struct into `Self`.
    The fields to be merged then need to implement `Clone`.
- `StructMergeIntoRef` trait.
    The counterpart of `StructMergeRef`, which merges `&Self` into a target struct.
    `StructMergeRef` is automatically implemented.

#### Derive Macro:

- `InterStruct` The main derive macro for this crate.

#### Derive Macro Attributes:

- `merge` attribute for generating `StructMergeInto` and the auto-implemented `StructMerge` implementations.
- `merge_ref` attribute for generating the `StructMergeRefInto` and the auto-implemented `StructMergeRef` implementations.
- `into` attribute for generating `std::convert::From` and the auto-implemented `std::convert::Into` implementations.
- `into_default` attribute for generating `std::convert::From` and the auto-implemented `std::convert::Into` implementations.
    This populates all non-matching fields by calling `Default::default` for the target struct.
