use crate::corpus::*;
use crate::sequence::*;
use crate::token::*;
use test_case::test_case;

pub fn meta_seqs() -> RefMap {
    let mut map = RefMap::new();
    map.insert(
        "ws*".to_string(),
        Box::new(NoneOrMoreSeq::new(
            Box::new(ChooseSeq::from_chars(" \t\n\r")),
            "".to_string(),
        )),
    );
    map.insert(
        "wordChar".to_string(),
        Box::new(ChooseSeq::from_chars(
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890",
        )),
    );
    map.insert(
        "word".to_string(),
        Box::new(OneOrMoreSeq::new(
            Box::new(RefSeq::new("wordChar".to_string())),
            "".to_string(),
        )),
    );
    map.insert(
        "rule".to_string(),
        Box::new(MultSeq::new(vec![
            (Box::new(RefSeq::new("ws*".to_string())), "".to_string()),
            (
                Box::new(RefSeq::new("word".to_string())),
                "name".to_string(),
            ),
            (Box::new(RefSeq::new("ws*".to_string())), "".to_string()),
            (Box::new(RawSeq::new("=".to_string())), "".to_string()),
            (Box::new(RefSeq::new("ws*".to_string())), "".to_string()),
            (Box::new(RefSeq::new("seq".to_string())), "seq".to_string()),
        ])),
    );
    map.insert(
        "seq".to_string(),
        Box::new(MultSeq::new(vec![
            (
                Box::new(RefSeq::new("noMultSeq".to_string())),
                "lhs".to_string(),
            ),
            (
                Box::new(RefSeq::new("optMultName".to_string())),
                "name".to_string(),
            ),
            (
                Box::new(NoneOrMoreSeq::new(
                    Box::new(MultSeq::new(vec![
                        (Box::new(RefSeq::new("ws*".to_string())), "".to_string()),
                        (Box::new(RawSeq::new("&".to_string())), "".to_string()),
                        (Box::new(RefSeq::new("ws*".to_string())), "".to_string()),
                        (
                            Box::new(RefSeq::new("noMultSeq".to_string())),
                            "seq".to_string(),
                        ),
                        (
                            Box::new(RefSeq::new("optMultName".to_string())),
                            "name".to_string(),
                        ),
                    ])),
                    "rhs's".to_string(),
                )),
                "rhs's".to_string(),
            ),
        ])),
    );
    map.insert(
        "noMultSeq".to_string(),
        Box::new(MultSeq::new(vec![
            (
                Box::new(RefSeq::new("noChooseSeq".to_string())),
                "lhs".to_string(),
            ),
            (
                Box::new(RefSeq::new("optName".to_string())),
                "name".to_string(),
            ),
            (
                Box::new(NoneOrMoreSeq::new(
                    Box::new(MultSeq::new(vec![
                        (Box::new(RefSeq::new("ws*".to_string())), "".to_string()),
                        (Box::new(RawSeq::new("|".to_string())), "".to_string()),
                        (Box::new(RefSeq::new("ws*".to_string())), "".to_string()),
                        (
                            Box::new(RefSeq::new("noChooseSeq".to_string())),
                            "seq".to_string(),
                        ),
                        (
                            Box::new(RefSeq::new("optName".to_string())),
                            "name".to_string(),
                        ),
                    ])),
                    "rhs's".to_string(),
                )),
                "rhs's".to_string(),
            ),
        ])),
    );
    map.insert(
        "noChooseSeq".to_string(),
        Box::new(MultSeq::new(vec![
            (
                Box::new(RefSeq::new("oneSeq".to_string())),
                "seq".to_string(),
            ),
            (
                Box::new(RefSeq::new("optName".to_string())),
                "name".to_string(),
            ),
            (
                Box::new(OptSeq::new(Box::new(ChooseSeq::from_chars("+*")))),
                "plus".to_string(),
            ),
        ])),
    );
    map.insert(
        "oneSeq".to_string(),
        Box::new(ChooseSeq::new(vec![
            (
                Box::new(RefSeq::new("rawSeq".to_string())),
                "raw".to_string(),
            ),
            (
                Box::new(RefSeq::new("parenSeq".to_string())),
                "paren".to_string(),
            ),
            (Box::new(RefSeq::new("word".to_string())), "ref".to_string()),
        ])),
    );
    map.insert(
        "rawSeq".to_string(),
        Box::new(MultSeq::new(vec![
            (Box::new(RawSeq::new("\'".to_string())), "".to_string()),
            (Box::new(AnySeq::new()), "token".to_string()),
            (Box::new(RawSeq::new("\'".to_string())), "".to_string()),
        ])),
    );
    map.insert(
        "parenSeq".to_string(),
        Box::new(MultSeq::new(vec![
            (Box::new(RawSeq::new("(".to_string())), "".to_string()),
            (Box::new(RefSeq::new("seq".to_string())), "seq".to_string()),
            (Box::new(RawSeq::new(")".to_string())), "".to_string()),
        ])),
    );
    map.insert(
        "optName".to_string(),
        Box::new(OptSeq::new(Box::new(MultSeq::new(vec![
            (Box::new(RawSeq::new(".".to_string())), "".to_string()),
            (
                Box::new(RefSeq::new("word".to_string())),
                "name".to_string(),
            ),
        ])))),
    );
    map.insert(
        "optMultName".to_string(),
        Box::new(OptSeq::new(Box::new(MultSeq::new(vec![
            (Box::new(RawSeq::new(":".to_string())), "".to_string()),
            (
                Box::new(RefSeq::new("word".to_string())),
                "name".to_string(),
            ),
        ])))),
    );
    map.insert(
        "main".to_string(),
        Box::new(NoneOrMoreSeq::new(
            Box::new(RefSeq::new("rule".to_string())),
            "rule".to_string(),
        )),
    );
    map
}

pub fn eval(text: &str) -> RefMap {
    let mut map = RefMap::new();
    let seqs = meta_seqs();
    let seq = seqs.get("main").unwrap();
    let matched = seq
        .match_corpus_first(&Corpus::make(text), &seqs)
        .unwrap()
        .new_token;
    for token in matched.get_children("rule") {
        let eval = eval_rule(&token);
        map.insert(eval.0, eval.1);
    }
    map
}

pub fn eval_rule(rule: &Token<'_>) -> (String, Box<dyn Sequence>) {
    let rule_name = rule.get_first_child("name").unwrap();
    let seq = eval_seq(&rule.get_first_child("seq").unwrap());
    (rule_name.content().to_string(), seq)
}

pub fn eval_seq(token: &Token<'_>) -> Box<dyn Sequence> {
    let seq = eval_no_mult_seq(&token.get_first_child("lhs").unwrap());
    let name = token
        .get_first_child("name")
        .unwrap()
        .get_first_child("name")
        .map_or_else(|| "".to_string(), |t| t.content().to_string());
    let mut to_ret = vec![(seq, name)];
    let rhs_s = token.get_first_child("rhs's").unwrap();
    let rhs_s = rhs_s.get_children("rhs's");
    for rhs in rhs_s {
        let seq = eval_no_mult_seq(&rhs.get_first_child("seq").unwrap());
        let name = rhs
            .get_first_child("name")
            .unwrap()
            .get_first_child("name")
            .map_or_else(|| "".to_string(), |t| t.content().to_string());
        to_ret.push((seq, name));
    }
    if to_ret.len() == 1 {
        to_ret.into_iter().next().unwrap().0
    } else {
        Box::new(MultSeq::new(to_ret))
    }
}

pub fn eval_no_mult_seq(token: &Token<'_>) -> Box<dyn Sequence> {
    let seq = eval_no_choose_seq(&token.get_first_child("lhs").unwrap());
    let name = token
        .get_first_child("name")
        .unwrap()
        .get_first_child("name")
        .map_or_else(|| "".to_string(), |t| t.content().to_string());
    let mut to_ret = vec![(seq, name)];
    let rhs_s = token.get_first_child("rhs's").unwrap();
    let rhs_s = rhs_s.get_children("rhs's");
    for rhs in rhs_s {
        let seq = eval_no_choose_seq(&rhs.get_first_child("seq").unwrap());
        let name = rhs
            .get_first_child("name")
            .unwrap()
            .get_first_child("name")
            .map_or_else(|| "".to_string(), |t| t.content().to_string());
        to_ret.push((seq, name));
    }
    if to_ret.len() == 1 {
        to_ret.into_iter().next().unwrap().0
    } else {
        Box::new(ChooseSeq::new(to_ret))
    }
}

pub fn eval_no_choose_seq(token: &Token<'_>) -> Box<dyn Sequence> {
    let seq = eval_one_seq(&token.get_first_child("seq").unwrap());
    let name = token
        .get_first_child("name")
        .unwrap()
        .get_first_child("name")
        .map_or_else(|| "".to_string(), |t| t.content().to_string());
    let plus = token.get_first_child("plus").unwrap().content() == "+";
    let many = token.get_first_child("plus").unwrap().content() == "*";
    if plus {
        Box::new(OneOrMoreSeq::new(seq, name))
    } else if many {
        Box::new(NoneOrMoreSeq::new(seq, name))
    } else {
        seq
    }
}

pub fn eval_one_seq(token: &Token<'_>) -> Box<dyn Sequence> {
    if let Some(raw) = token.get_first_child("raw") {
        Box::new(RawSeq::new(
            raw.get_first_child("token").unwrap().content().to_string(),
        ))
    } else if let Some(ref_name) = token.get_first_child("ref") {
        Box::new(RefSeq::new(ref_name.content().to_string()))
    } else if let Some(seq_t) = token.get_first_child("paren") {
        eval_seq(&seq_t.get_first_child("seq").unwrap())
    } else {
        unimplemented!()
    }
}

#[test_case("
main = 'a'
", "a";
"one raw rule")]
#[test_case("
main = ('a')
", "a";
"paren rule")]
#[test_case("
main = 'a'
", "b";
"one bad raw rule")]
#[test_case("
a = 'a'
main = a
", "a";
"one ref rule")]
#[test_case("
main = 'a':hi & 'b'
", "ab";
"one mult rule")]
#[test_case("
digit = '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0'
main = digit+
", "1205";
"one or more rule")]
#[test_case("
main = 'a':hi & 'b'*
", "a";
"mult and one or more")]
pub fn test_eval(rules: &str, text: &str) {
    let seqs = eval(rules);
    let seq = seqs.get("main").unwrap();
    let matched = seq.match_corpus_first(&Corpus::make(text), &seqs);
    println!("{matched:?}");
}
