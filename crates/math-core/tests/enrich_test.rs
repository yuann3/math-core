use math_core::{LatexToMathML, MathCoreConfig, MathDisplay};

#[test]
fn test_unmatched_openers_take_linear_time() {
    let converter = LatexToMathML::new(MathCoreConfig::default(), vec![]).unwrap();
    // Each unmatched opener used to double the time: 40 took hours.
    let latex = r"[ \sin x ".repeat(40);
    let mathml = converter
        .convert_with_local_state(&latex, MathDisplay::Inline)
        .unwrap()
        .mathml;
    // Past the fuel the formula keeps its nodes, without invisible operators.
    assert_eq!(mathml.matches(">[</mo>").count(), 40);
    assert_eq!(mathml.matches(">sin<").count(), 40);
    // A few unmatched openers are within the fuel, and still enriched.
    let mathml = converter
        .convert_with_local_state(r"[ [ [ \sin x", MathDisplay::Inline)
        .unwrap()
        .mathml;
    assert!(mathml.contains("&ApplyFunction;"), "{mathml}");
}
