use super::*;

#[test]
pub fn mult_test() {
    let seq = MultSeq::new(vec![
        (Box::new(RawSeq::new("a".to_string())), "hi".to_string()),
        (Box::new(RawSeq::new("b".to_string())), "".to_string()),
    ]);
    let new_ref_map = &RefMap::new();
    seq.assert_matches(&Corpus::make("baba"), new_ref_map, TokenMatchTestType::None);
    seq.assert_matches(
        &Corpus::make("ababa"),
        new_ref_map,
        TokenMatchTestType::First,
    );
}

#[test]
pub fn opt_test() {
    let seq = MultSeq::new(vec![
        (Box::new(RawSeq::new("a".to_string())), "hi".to_string()),
        (Box::new(RawSeq::new("b".to_string())), "".to_string()),
        (
            Box::new(OptSeq::new(Box::new(RawSeq::new("c".to_string())))),
            "".to_string(),
        ),
    ]);
    let new_ref_map = &RefMap::new();
    seq.assert_matches(
        &Corpus::make("ababa"),
        new_ref_map,
        TokenMatchTestType::First,
    );
    seq.assert_matches(
        &Corpus::make("abcaba"),
        new_ref_map,
        TokenMatchTestType::First,
    );
    seq.assert_matches(
        &Corpus::make("acbcaba"),
        new_ref_map,
        TokenMatchTestType::None,
    );
}

#[test]
pub fn none_or_more_test() {
    let seq = MultSeq::new(vec![
        (Box::new(RawSeq::new("a".to_string())), "hi".to_string()),
        (Box::new(RawSeq::new("b".to_string())), "".to_string()),
        (
            Box::new(NoneOrMoreSeq::new(
                Box::new(RawSeq::new("c".to_string())),
                "oh boy".to_string(),
            )),
            "".to_string(),
        ),
    ]);
    let new_ref_map = &RefMap::new();
    seq.assert_matches(
        &Corpus::make("ababa"),
        new_ref_map,
        TokenMatchTestType::First,
    );
    seq.assert_matches(
        &Corpus::make("abcaba"),
        new_ref_map,
        TokenMatchTestType::First,
    );
    seq.assert_matches(
        &Corpus::make("abccccccccaba"),
        new_ref_map,
        TokenMatchTestType::None,
    );
    seq.assert_matches(
        &Corpus::make("acbcaba"),
        new_ref_map,
        TokenMatchTestType::None,
    );
}

#[test]
pub fn none_or_more_and_mult_test() {
    let seq = MultSeq::new(vec![
        (Box::new(RawSeq::new("a".to_string())), "hi".to_string()),
        (Box::new(RawSeq::new("b".to_string())), "".to_string()),
        (
            Box::new(NoneOrMoreSeq::new(
                Box::new(MultSeq::new(vec![
                    (Box::new(RawSeq::new("a".to_string())), "hi".to_string()),
                    (Box::new(RawSeq::new("b".to_string())), "".to_string()),
                ])),
                "inner".to_string(),
            )),
            "mult".to_string(),
        ),
    ]);
    let new_ref_map = &RefMap::new();
    seq.assert_matches(
        &Corpus::make("ababa"),
        new_ref_map,
        TokenMatchTestType::First,
    );
    seq.assert_matches(
        &Corpus::make("abcaba"),
        new_ref_map,
        TokenMatchTestType::First,
    );
    seq.assert_matches(
        &Corpus::make("abccccccccaba"),
        new_ref_map,
        TokenMatchTestType::First,
    );
    seq.assert_matches(
        &Corpus::make("acbcaba"),
        new_ref_map,
        TokenMatchTestType::None,
    );
}

#[test]
pub fn one_or_more_test() {
    let seq = MultSeq::new(vec![
        (Box::new(RawSeq::new("a".to_string())), "hi".to_string()),
        (Box::new(RawSeq::new("b".to_string())), "".to_string()),
        (
            Box::new(OneOrMoreSeq::new(
                Box::new(MultSeq::new(vec![
                    (Box::new(RawSeq::new("a".to_string())), "hi".to_string()),
                    (Box::new(RawSeq::new("b".to_string())), "".to_string()),
                ])),
                "inner".to_string(),
            )),
            "mult".to_string(),
        ),
    ]);
    let new_ref_map = &RefMap::new();
    seq.assert_matches(
        &Corpus::make("ababa"),
        new_ref_map,
        TokenMatchTestType::First,
    );
    seq.assert_matches(
        &Corpus::make("abcaba"),
        new_ref_map,
        TokenMatchTestType::None,
    );
    seq.assert_matches(
        &Corpus::make("abccccccccaba"),
        new_ref_map,
        TokenMatchTestType::None,
    );
    seq.assert_matches(
        &Corpus::make("acbcaba"),
        new_ref_map,
        TokenMatchTestType::None,
    );
}

#[test]
pub fn choose_test() {
    let seq = ChooseSeq::new(vec![
        (Box::new(RawSeq::new("a".to_string())), "first".to_string()),
        (Box::new(RawSeq::new("b".to_string())), "second".to_string()),
        (Box::new(RawSeq::new("c".to_string())), "third".to_string()),
    ]);
    let new_ref_map = &RefMap::new();
    seq.assert_matches(&Corpus::make("a"), new_ref_map, TokenMatchTestType::All);
    seq.assert_matches(&Corpus::make("b"), new_ref_map, TokenMatchTestType::All);
    seq.assert_matches(&Corpus::make("c"), new_ref_map, TokenMatchTestType::All);
    seq.assert_matches(&Corpus::make("d"), new_ref_map, TokenMatchTestType::None);
    seq.assert_matches(&Corpus::make(""), new_ref_map, TokenMatchTestType::None);
}
