use super::*;
use crate::token::*;

#[test]
pub fn mult_test() {
    let seq = MultSeq {
        seqs: vec![
            (
                Box::new(RawSeq::new("a".to_string())),
                Some("hi".to_string()),
            ),
            (Box::new(RawSeq::new("b".to_string())), None),
        ],
    };
    let matched = dbg!(seq.match_tokens(Corpus::make("ababa").tokens.as_slice()));
    assert!(matched.is_some());
    let matched = dbg!(seq.match_tokens(Corpus::make("baba").tokens.as_slice()));
    assert!(matched.is_none());
}

#[test]
pub fn opt_test() {
    let seq = MultSeq {
        seqs: vec![
            (
                Box::new(RawSeq::new("a".to_string())),
                Some("hi".to_string()),
            ),
            (Box::new(RawSeq::new("b".to_string())), None),
            (
                Box::new(OptSeq::new(Box::new(RawSeq::new("c".to_string())))),
                None,
            ),
        ],
    };
    let matched = dbg!(seq.match_tokens(Corpus::make("ababa").tokens.as_slice()));
    assert!(matched.is_some());
    let matched = dbg!(seq.match_tokens(Corpus::make("abcaba").tokens.as_slice()));
    assert!(matched.is_some());
    let matched = dbg!(seq.match_tokens(Corpus::make("acbcaba").tokens.as_slice()));
    assert!(matched.is_none());
}

#[test]
pub fn none_or_more_test() {
    let seq = MultSeq {
        seqs: vec![
            (
                Box::new(RawSeq::new("a".to_string())),
                Some("hi".to_string()),
            ),
            (Box::new(RawSeq::new("b".to_string())), None),
            (
                Box::new(NoneOrMoreSeq::new(
                    Box::new(RawSeq::new("c".to_string())),
                    Some("oh boy".to_string()),
                )),
                None,
            ),
        ],
    };
    let matched = dbg!(seq.match_tokens(Corpus::make("ababa").tokens.as_slice()));
    assert!(matched.is_some());
    let matched = dbg!(seq.match_tokens(Corpus::make("abcaba").tokens.as_slice()));
    assert!(matched.is_some());
    let matched = dbg!(seq.match_tokens(Corpus::make("abccccccccaba").tokens.as_slice()));
    assert!(matched.is_some());
    let matched = dbg!(seq.match_tokens(Corpus::make("acbcaba").tokens.as_slice()));
    assert!(matched.is_none());
}

#[test]
pub fn none_or_more_and_mult_test() {
    let seq = MultSeq::new(vec![
        (
            Box::new(RawSeq::new("a".to_string())),
            Some("hi".to_string()),
        ),
        (Box::new(RawSeq::new("b".to_string())), None),
        (
            Box::new(NoneOrMoreSeq::new(
                Box::new(MultSeq::new(vec![
                    (
                        Box::new(RawSeq::new("a".to_string())),
                        Some("hi".to_string()),
                    ),
                    (Box::new(RawSeq::new("b".to_string())), None),
                ])),
                Some("inner".to_string()),
            )),
            Some("mult".to_string()),
        ),
    ]);
    let matched = dbg!(seq.match_tokens(Corpus::make("ababa").tokens.as_slice()));
    assert!(matched.is_some());
    let matched = dbg!(seq.match_tokens(Corpus::make("abcaba").tokens.as_slice()));
    assert!(matched.is_some());
    let matched = dbg!(seq.match_tokens(Corpus::make("abccccccccaba").tokens.as_slice()));
    assert!(matched.is_some());
    let matched = dbg!(seq.match_tokens(Corpus::make("acbcaba").tokens.as_slice()));
    assert!(matched.is_none());
}
