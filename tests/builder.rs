use trybuild::TestCases;

#[test]
fn builder() {
    let t = TestCases::new();
    t.pass("tests/builder/basic.rs");
    t.pass("tests/builder/default.rs");
    t.compile_fail("tests/builder/default-missing.rs");
}
