// Check that we can parse and deserialize every test file.
#[test]
fn parse_test_files() {
    let test_files_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("test_files");
    assert!(test_files_path.exists());
    assert!(test_files_path.is_dir());
    for entry in std::fs::read_dir(test_files_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let ext = path.extension().and_then(|s| s.to_str());
        if ext == Some("fs") || ext == Some("vs") {
            let glsl_str = std::fs::read_to_string(&path).unwrap();
            let _isf = match isf::parse(&glsl_str) {
                // Ignore non-ISF vertex shaders.
                Err(isf::ParseError::MissingTopComment) if ext == Some("vs") => continue,
                Err(err) => panic!("err while parsing {}: {}", path.display(), err),
                Ok(isf) => isf,
            };
        }
    }
}

// An unknown input type is an error, not a panic: shader collections are full
// of types no one has heard of, and one of them must not take the host down.
#[test]
fn unknown_input_type_is_an_error() {
    let src = r#"/*{ "INPUTS": [ { "NAME": "x", "TYPE": "spline" } ] }*/"#;
    assert!(isf::parse(src).is_err());
}

// MadMapper writes `int` where the spec says `long`.
#[test]
fn int_is_read_as_long() {
    let src = r#"/*{ "INPUTS": [ { "NAME": "x", "TYPE": "int", "DEFAULT": 5 } ] }*/"#;
    let isf = isf::parse(src).unwrap();
    assert!(matches!(isf.inputs[0].ty, isf::InputType::Long(_)));
}
