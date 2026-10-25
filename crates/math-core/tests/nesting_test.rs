use math_core::{LatexToMathML, MathCoreConfig, MathDisplay};

#[test]
fn test_deep_nesting_is_an_error() {
    // An unoptimised build takes about 10 MiB of stack at the limit.
    std::thread::Builder::new()
        .stack_size(32 << 20)
        .spawn(deep_nesting)
        .unwrap()
        .join()
        .unwrap();
}

fn deep_nesting() {
    let converter = LatexToMathML::new(MathCoreConfig::default(), vec![]).unwrap();
    let inputs = [
        "{".repeat(100_000),
        r"\frac{".repeat(100_000),
        r"\sqrt".repeat(100_000) + "x",
        r"\left(".repeat(100_000),
        r"x^{".repeat(100_000),
        r"\begin{matrix}".repeat(100_000),
        r"\sqrt[".repeat(100_000),
        r"\overset{".repeat(100_000),
    ];
    for latex in &inputs {
        let Err(err) = converter.convert_with_local_state(latex, MathDisplay::Block) else {
            panic!("{} converted", &latex[..20]);
        };
        assert!(
            err.error_message().contains("limit"),
            "{}: {}",
            &latex[..20],
            err.error_message()
        );
    }
    // Nesting up to the limit still converts.
    let ok = "{".repeat(200) + "x" + &"}".repeat(200);
    assert!(
        converter
            .convert_with_local_state(&ok, MathDisplay::Inline)
            .is_ok()
    );
}
