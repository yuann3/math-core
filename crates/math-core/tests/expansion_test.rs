use math_core::{LatexToMathML, MathCoreConfig, MathDisplay};

fn converter() -> LatexToMathML {
    let config = MathCoreConfig {
        global_group: true,
        ..Default::default()
    };
    LatexToMathML::new(config, vec![]).unwrap()
}

fn too_many(r: &Result<math_core::ConvertResult, Box<math_core::LatexError>>) -> bool {
    matches!(r, Err(e) if e.error_message().starts_with("Too many expansions"))
}

#[test]
fn test_a_long_body_expanded_many_times_is_an_error() {
    // 900 expansions each, under `MaxExpansions`, of a body of 10,000 tokens.
    let body = "a".repeat(10_000);
    let mut snippets = vec![(
        format!("\\newcommand{{\\x}}{{{body}}}"),
        MathDisplay::Inline,
    )];
    snippets.push(("\\x".repeat(900), MathDisplay::Inline));
    let results = converter().convert_all(&snippets);
    assert!(results[0].is_ok());
    assert!(too_many(&results[1]));
}

#[test]
fn test_the_budget_is_shared_by_the_snippets() {
    // Each small snippet expands the long body once, which one snippet alone may do, and
    // the definition pays for eight of them.
    let body = "a".repeat(5_000);
    let mut snippets = vec![(
        format!("\\newcommand{{\\x}}{{{body}}}"),
        MathDisplay::Inline,
    )];
    snippets.extend((0..20).map(|_| ("\\x".to_string(), MathDisplay::Inline)));
    let results = converter().convert_all(&snippets);
    assert!(results[..2].iter().all(Result::is_ok));
    assert!(results.iter().any(too_many));
}

#[test]
fn test_ordinary_macros_are_well_within_the_budget() {
    let macros = vec![
        ("R".to_string(), r"\mathbb{R}".to_string()),
        (
            "norm".to_string(),
            r"\left\lVert #1 \right\rVert".to_string(),
        ),
    ];
    let converter = LatexToMathML::new(MathCoreConfig::default(), macros).unwrap();
    let latex = r"\norm{x}_{\R} \leq \norm{\norm{y}}".repeat(200);
    assert!(
        converter
            .convert_with_local_state(&latex, MathDisplay::Block)
            .is_ok()
    );
}
