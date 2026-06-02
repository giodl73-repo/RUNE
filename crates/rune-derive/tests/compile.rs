#[test]
fn rune_contract_macro_compile_checks() {
    let tests = trybuild::TestCases::new();
    tests.pass("tests/ui/pass_contract.rs");
    tests.compile_fail("tests/ui/fail_unsupported_attribute.rs");
    tests.compile_fail("tests/ui/fail_missing_id.rs");
    tests.compile_fail("tests/ui/fail_missing_version.rs");
}
