use crate::corpus::*;
use crate::meta::*;
use crate::sequence::*;
use crate::token::*;
use test_case::test_case;

pub fn calc_seqs() -> RefMap {
    eval_rule_set(
        "
    nonzero = 1..9
    digit = 0..9
    posInt = nonzero:first & digit*
    oper = [+-]
    multOper = [*/]
    ws_s = [ \t\n\r]*
    expr = 
        multExpr:lhs & 
        (ws_s & oper:oper & ws_s & multExpr:oper).rhs_s*:rhs_s
    multExpr = 
        numExpr:lhs & 
        (ws_s & multOper:oper & ws_s & numExpr:oper).rhs_s*:rhs_s
    numExpr = posInt
    ",
    )
}

pub fn eval(body: &str) -> Option<f64> {
    let seqs = calc_seqs();
    let seq = seqs.get("expr").unwrap();
    let matched = seq.match_tokens(&Corpus::make(body).tokens, &seqs);
    matched.map(|t| eval_expr(&t.new_token))
}

pub fn eval_expr(expr: &Token<'_>) -> f64 {
    let mut to_ret = eval_mult_expr(&expr.get_first_child("lhs").unwrap());
    for opers in expr.get_first_child("rhs_s").unwrap().get_children("rhs_s") {
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
    for opers in expr.get_first_child("rhs_s").unwrap().get_children("rhs_s") {
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
