use crate::corpus::*;
use crate::sequence::*;
use crate::token::*;
use test_case::test_case;

pub fn calc_seqs() -> RefMap {
    let mut map = RefMap::new();
    let nonzero_seq = ChooseSeq::from_chars("123456789");
    map.insert(
        "nonzero".to_string(),
        Box::new(nonzero_seq) as Box<dyn Sequence>,
    );
    map.insert(
        "digit".to_string(),
        Box::new(ChooseSeq::from_chars("1234567890")),
    );
    map.insert(
        "posInt".to_string(),
        Box::new(MultSeq::new(vec![
            (
                Box::new(RefSeq::new("nonzero".to_string())),
                "firstDigit".to_string(),
            ),
            (
                Box::new(NoneOrMoreSeq::new(
                    Box::new(RefSeq::new("digit".to_string())),
                    "".to_string(),
                )),
                "".to_string(),
            ),
        ])),
    );
    map.insert("oper".to_string(), Box::new(ChooseSeq::from_chars("+-")));
    map.insert(
        "multOper".to_string(),
        Box::new(ChooseSeq::from_chars("/*")),
    );
    map.insert(
        "ws*".to_string(),
        Box::new(NoneOrMoreSeq::new(
            Box::new(ChooseSeq::from_chars(" \t\n\r")),
            "".to_string(),
        )),
    );
    map.insert(
        "expr".to_string(),
        Box::new(MultSeq::new(vec![
            (
                Box::new(RefSeq::new("multExpr".to_string())),
                "lhs".to_string(),
            ),
            (
                Box::new(NoneOrMoreSeq::new(
                    Box::new(MultSeq::new(vec![
                        (Box::new(RefSeq::new("ws*".to_string())), "".to_string()),
                        (
                            Box::new(RefSeq::new("oper".to_string())),
                            "oper".to_string(),
                        ),
                        (Box::new(RefSeq::new("ws*".to_string())), "".to_string()),
                        (
                            Box::new(RefSeq::new("multExpr".to_string())),
                            "oper".to_string(),
                        ),
                    ])),
                    "rhs's".to_string(),
                )),
                "rhs's".to_string(),
            ),
        ])),
    );
    map.insert(
        "multExpr".to_string(),
        Box::new(MultSeq::new(vec![
            (
                Box::new(RefSeq::new("numExpr".to_string())),
                "lhs".to_string(),
            ),
            (
                Box::new(NoneOrMoreSeq::new(
                    Box::new(MultSeq::new(vec![
                        (Box::new(RefSeq::new("ws*".to_string())), "".to_string()),
                        (
                            Box::new(RefSeq::new("multOper".to_string())),
                            "oper".to_string(),
                        ),
                        (Box::new(RefSeq::new("ws*".to_string())), "".to_string()),
                        (
                            Box::new(RefSeq::new("numExpr".to_string())),
                            "oper".to_string(),
                        ),
                    ])),
                    "rhs's".to_string(),
                )),
                "rhs's".to_string(),
            ),
        ])),
    );
    map.insert(
        "numExpr".to_string(),
        Box::new(RefSeq::new("posInt".to_string())),
    );
    map
}

pub fn eval(body: &str) -> Option<f64> {
    let seqs = calc_seqs();
    let seq = seqs.get("expr").unwrap();
    let matched = seq.match_tokens(&Corpus::make(body).tokens, &seqs);
    matched.map(|t| eval_expr(&t.new_token))
}

pub fn eval_expr(expr: &Token<'_>) -> f64 {
    let mut to_ret = eval_mult_expr(&expr.get_first_child("lhs").unwrap());
    for opers in expr.get_first_child("rhs's").unwrap().get_children("rhs's") {
        match opers.get_children("oper")[0].content() {
            "+" => to_ret += eval_mult_expr(&opers.get_children("oper")[1]),
            "-" => to_ret -= eval_mult_expr(&opers.get_children("oper")[1]),
            _ => {}
        }
    }
    to_ret
}

pub fn eval_mult_expr(expr: &Token<'_>) -> f64 {
    let mut to_ret = eval_num_expr(&expr.get_first_child("lhs").unwrap());
    for opers in expr.get_first_child("rhs's").unwrap().get_children("rhs's") {
        match opers.get_children("oper")[0].content() {
            "*" => to_ret *= eval_num_expr(&opers.get_children("oper")[1]),
            "/" => to_ret /= eval_num_expr(&opers.get_children("oper")[1]),
            _ => {}
        }
    }
    to_ret
}

pub fn eval_num_expr(expr: &Token<'_>) -> f64 {
    str::parse(expr.content()).unwrap()
}

#[test]
pub fn int_test() {
    let seqs = calc_seqs();
    let seq = seqs.get("posInt").unwrap();
    seq.assert_matches(&Corpus::make("1"), &seqs, TokenMatchTestType::All);
    seq.assert_matches(&Corpus::make("1234"), &seqs, TokenMatchTestType::All);
    seq.assert_matches(&Corpus::make("0123"), &seqs, TokenMatchTestType::None);
    seq.assert_matches(&Corpus::make("aba"), &seqs, TokenMatchTestType::None);
}

#[test]
pub fn expr_test() {
    let seqs = calc_seqs();
    let seq = seqs.get("expr").unwrap();
    seq.assert_matches(&Corpus::make("1+1"), &seqs, TokenMatchTestType::All);
    seq.assert_matches(
        &Corpus::make("1683 / 4963 * 2"),
        &seqs,
        TokenMatchTestType::All,
    );
    seq.assert_matches(&Corpus::make("04 * 5"), &seqs, TokenMatchTestType::None);
    seq.assert_matches(&Corpus::make("6 + + 2"), &seqs, TokenMatchTestType::First);
}

#[test_case("1", Some(1.0); "number 1")]
#[test_case("1234", Some(1234.0); "number 1234")]
#[test_case("1 + 2", Some(3.0); "number 1 + 2")]
#[test_case("1 + 2 / 2", Some(2.0); "number two")]
#[test_case("1 / 2", Some(0.5); "number one-half")]
pub fn eval_test(text: &str, expected: Option<f64>) {
    assert_eq!(eval(text), expected);
}
