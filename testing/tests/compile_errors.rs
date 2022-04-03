use std::path::PathBuf;

#[test]
fn compile_error() {
    let t = trybuild::TestCases::new();

    // `trybuild` works like this:
    // - Create a stub testing crate in `target/tests/$CARGO_PKG_NAME` which I'll call `$TESTDIR`.
    // - Compile each included test fail as if it was a file in
    //   the `$TESTDIR/bin/` folder. E.g. `$TESTDIR/bin/$TESTNAME.rs`.
    //
    // InterStruct however does module resolution by looking at the `$TESTDIR/src` folder.
    //
    // This means that `crate::Struct` is interpreted in different ways by the compiler and
    // InterStruct.
    //
    // - Cargo sees the actual binary file as it's module resolution root, i.e.
    //   `$TESTDIR/bin/$TESTNAME`.
    // - Interstruct thinks that it's located in `$TESTDIR/src/lib.rs`.
    //
    // Since this is not a problem we can resolve (yet), a copy of all structs used as
    // targets in the InterStruct test files is copied to `$TESTDIR/src/lib.rs`.
    //
    // Otherwise InterStruct won't find the struct declaration it expects to find.
    let target_dir = PathBuf::from("../target/tests/testing/src");
    std::fs::create_dir_all(target_dir).expect("Failed to create testing project dir");

    let stub_declarations = PathBuf::from("./tests/stub_declarations/lib.rs");
    std::fs::copy(stub_declarations, "../target/tests/testing/src/lib.rs")
        .expect("Failed to copy stub lib.rs");

    // Uncomment if you want to test a special
    //let single = Some("tests/merge/incompatible_type.rs".to_string());
    let single = None::<String>;
    if let Some(single) = single {
        t.compile_fail(single);
    } else {
        t.compile_fail("tests/into/*.rs");
        t.compile_fail("tests/merge/*.rs");
        t.compile_fail("tests/path/*.rs");
    }
}
