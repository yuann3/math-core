use math_core::{Label, LatexToMathML, MathCoreConfig, MathDisplay, Reference};

fn label(name: &str, span: std::ops::Range<usize>, tag: Option<&str>) -> Label {
    Label {
        name: name.into(),
        span,
        tag: tag.map(Into::into),
    }
}

#[test]
fn test_labels_and_references_per_snippet() {
    let converter = LatexToMathML::new(MathCoreConfig::default(), vec![]).unwrap();
    let snippets = [
        (r"\eqref{b} \eqref{nope}", MathDisplay::Inline),
        (
            r"\begin{align} x \label{a} \\ y \notag \label{n} \\ z \label{b} \end{align}",
            MathDisplay::Block,
        ),
        (
            r"\begin{equation*} w \label{s} \end{equation*}",
            MathDisplay::Block,
        ),
        (
            r"\begin{equation} v \tag{A} \label{a} \end{equation}",
            MathDisplay::Block,
        ),
    ];
    let results: Vec<_> = converter
        .convert_all(&snippets)
        .into_iter()
        .map(Result::unwrap)
        .collect();
    assert_eq!(
        results[0].references,
        [
            Reference {
                name: "b".into(),
                span: 0..9
            },
            Reference {
                name: "nope".into(),
                span: 10..22
            },
        ]
    );
    // A forward reference still finds its number.
    assert!(results[0].mathml.contains("(2)"));
    assert_eq!(
        results[1].labels,
        [
            label("a", 16..25, Some("1")),
            label("n", 38..47, None),
            label("b", 53..62, Some("2")),
        ]
    );
    assert_eq!(results[2].labels, [label("s", 20..29, None)]);
    // A label given again is listed again, and the tag is the equation's own.
    assert_eq!(results[3].labels, [label("a", 27..36, Some("A"))]);
    assert!(
        results
            .iter()
            .all(|r| r.references.is_empty() || r.labels.is_empty())
    );
}

#[test]
fn test_labels_with_global_state() {
    let mut converter = LatexToMathML::new(MathCoreConfig::default(), vec![]).unwrap();
    let first = converter
        .convert_with_global_state(
            r"\begin{equation} x \label{a} \end{equation}",
            MathDisplay::Block,
        )
        .unwrap();
    let second = converter
        .convert_with_global_state(
            r"\begin{equation} y \label{b} \end{equation}",
            MathDisplay::Block,
        )
        .unwrap();
    assert_eq!(first.labels, [label("a", 19..28, Some("1"))]);
    assert_eq!(second.labels, [label("b", 19..28, Some("2"))]);
}
