use trybuild::TestCases;

#[test]
fn builder() {
    let t = TestCases::new();
    t.pass("tests/01-basic.rs");
    t.pass("tests/02-default.rs");
    t.compile_fail("tests/03-default-missing.rs");
}
