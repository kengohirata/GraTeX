use gratex::cli::convert::{convert_input, InputType};
use rstest::rstest;
use std::path::{Path, PathBuf};

fn fixture_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

fn assert_fixture_conversion(test_case: &str) {
    let input = format!("{}.tex", test_case);
    let expected = format!("{}.out", test_case);
    let input_path = fixture_path(&input);
    let expected_output =
        std::fs::read_to_string(fixture_path(&expected)).expect("expected output missing");
    let result =
        convert_input(InputType::File { path: input_path }).expect("conversion for fixture failed");
    assert_eq!(
        result, expected_output,
        "conversion output did not match expected contents"
    );
}

#[rstest]
#[case("test_math")]
#[case("test_inline_math")]
#[case("test_environment")]
fn converts_fixture(#[case] test_case: &str) {
    assert_fixture_conversion(test_case);
}
