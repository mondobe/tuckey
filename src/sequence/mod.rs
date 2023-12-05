use crate::corpus::*;
use crate::token::*;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct TokenMatch<'a> {
    pub len: usize,
    pub new_token: Token<'a>,
}

pub trait Sequence {
    fn match_tokens<'a>(&'a self, tokens: &[Token<'a>]) -> Option<TokenMatch>;
}

#[derive(Clone, PartialEq)]
pub struct RawSeq {
    pub target: String,
}

impl Sequence for RawSeq {
    fn match_tokens<'a>(&'a self, tokens: &[Token<'a>]) -> Option<TokenMatch> {
        tokens.first().and_then(move |t| {
            if t.content() == self.target {
                Some(TokenMatch {
                    len: 1,
                    new_token: t.clone(),
                })
            } else {
                None
            }
        })
    }
}

impl RawSeq {
    fn new(target: String) -> Self {
        RawSeq { target }
    }
}

pub struct MultSeq {
    pub seqs: Vec<(Box<dyn Sequence>, Option<String>)>,
}

impl Sequence for MultSeq {
    fn match_tokens<'a>(&'a self, tokens: &[Token<'a>]) -> Option<TokenMatch> {
        let mut match_index = 0;
        let mut children = vec![];
        for (seq, key) in &self.seqs {
            if let Some(matched) = seq.match_tokens(&tokens[match_index..]) {
                if let Some(key_that_exists) = key {
                    children.push((key_that_exists.to_string(), matched.new_token));
                }
                match_index += matched.len;
            } else {
                return None;
            }
        }
        Some(TokenMatch {
            len: match_index,
            new_token: Token {
                source: tokens[0].source,
                data: TokenData::Branch(children),
            },
        })
    }
}

impl MultSeq {
    fn new(seqs: Vec<(Box<dyn Sequence>, Option<String>)>) -> Self {
        MultSeq { seqs }
    }
}

pub struct OptSeq {
    pub seq: Box<dyn Sequence>,
}

impl Sequence for OptSeq {
    fn match_tokens<'a>(&'a self, tokens: &[Token<'a>]) -> Option<TokenMatch> {
        match self.seq.match_tokens(tokens) {
            Some(did_match) => Some(did_match),
            None => Some(TokenMatch {
                len: 0,
                new_token: Token {
                    source: tokens[0].source,
                    data: TokenData::Leaf(0..0),
                },
            }),
        }
    }
}

impl OptSeq {
    fn new(seq: Box<dyn Sequence>) -> Self {
        OptSeq { seq }
    }
}

pub struct NoneOrMoreSeq {
    pub match_name: Option<String>,
    pub seq: Box<dyn Sequence>,
}

impl Sequence for NoneOrMoreSeq {
    fn match_tokens<'a>(&'a self, tokens: &[Token<'a>]) -> Option<TokenMatch> {
        let mut match_index = 0;
        let mut children = vec![];
        loop {
            match self.seq.match_tokens(&tokens[match_index..]) {
                Some(matched) => {
                    if let Some(name) = &self.match_name {
                        children.push((name.to_string(), matched.new_token));
                    }
                    match_index += matched.len;
                }
                None => {
                    break;
                }
            };
        }
        Some(TokenMatch {
            len: match_index,
            new_token: Token {
                source: tokens[0].source,
                data: TokenData::Branch(children),
            },
        })
    }
}

impl NoneOrMoreSeq {
    fn new(seq: Box<dyn Sequence>, match_name: Option<String>) -> Self {
        NoneOrMoreSeq { seq, match_name }
    }
}
