use std::collections::HashMap;

use crate::corpus::*;
use crate::token::*;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct TokenMatch<'a> {
    pub len: usize,
    pub new_token: Token<'a>,
}

pub type RefMap = HashMap<String, Box<dyn Sequence>>;

pub enum TokenMatchTestType {
    None,
    First,
    All,
}

pub trait Sequence {
    fn match_tokens<'a>(&'a self, tokens: &[Token<'a>], refs: &'a RefMap) -> Option<TokenMatch>;

    fn match_corpus_first<'a>(
        &'a self,
        corpus: &Corpus<'a>,
        refs: &'a RefMap,
    ) -> Option<TokenMatch> {
        self.match_tokens(&corpus.tokens, refs)
    }

    fn assert_matches<'a>(
        &'a self,
        corpus: &Corpus<'a>,
        refs: &'a RefMap,
        should_match: TokenMatchTestType,
    ) {
        let matched = dbg!(self.match_corpus_first(corpus, refs));
        match should_match {
            TokenMatchTestType::None => {
                assert!(matched.is_none());
            }
            TokenMatchTestType::First => {
                assert!(matched.is_some());
            }
            TokenMatchTestType::All => {
                assert!(matched.is_some_and(|t| t.len == corpus.tokens.len()));
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct RawSeq {
    pub target: String,
}

impl Sequence for RawSeq {
    fn match_tokens<'a>(&'a self, tokens: &[Token<'a>], _: &'a RefMap) -> Option<TokenMatch> {
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
    pub fn new(target: String) -> Self {
        Self { target }
    }
}

pub struct MultSeq {
    pub seqs: Vec<(Box<dyn Sequence>, String)>,
}

impl Sequence for MultSeq {
    fn match_tokens<'a>(&'a self, tokens: &[Token<'a>], refs: &'a RefMap) -> Option<TokenMatch> {
        let mut match_index = 0;
        let mut children = vec![];
        for (seq, key) in &self.seqs {
            if let Some(matched) = seq.match_tokens(&tokens[match_index..], refs) {
                children.push((key.to_string(), matched.new_token));
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
    pub fn new(seqs: Vec<(Box<dyn Sequence>, String)>) -> Self {
        Self { seqs }
    }
}

pub struct OptSeq {
    pub seq: Box<dyn Sequence>,
    pub match_name: String,
}

impl Sequence for OptSeq {
    fn match_tokens<'a>(&'a self, tokens: &[Token<'a>], refs: &'a RefMap) -> Option<TokenMatch> {
        match self.seq.match_tokens(tokens, refs) {
            Some(did_match) => Some(TokenMatch {
                len: did_match.len,
                new_token: Token {
                    source: tokens[0].source,
                    data: TokenData::Branch(vec![(self.match_name.clone(), did_match.new_token)]),
                },
            }),
            None => Some(TokenMatch {
                len: 0,
                new_token: Token {
                    source: tokens[0].source,
                    data: TokenData::Branch(vec![]),
                },
            }),
        }
    }
}

impl OptSeq {
    pub fn new(seq: Box<dyn Sequence>, match_name: String) -> Self {
        Self { seq, match_name }
    }
}

pub struct NoneOrMoreSeq {
    pub match_name: String,
    pub seq: Box<dyn Sequence>,
}

impl Sequence for NoneOrMoreSeq {
    fn match_tokens<'a>(&'a self, tokens: &[Token<'a>], refs: &'a RefMap) -> Option<TokenMatch> {
        let mut match_index = 0;
        let mut children = vec![];
        while let Some(matched) = self.seq.match_tokens(&tokens[match_index..], refs) {
            children.push((self.match_name.to_string(), matched.new_token));
            match_index += matched.len;
        }
        Some(TokenMatch {
            len: match_index,
            new_token: Token {
                source: tokens.get(0).map_or_else(|| "", |t| t.source),
                data: TokenData::Branch(children),
            },
        })
    }
}

impl NoneOrMoreSeq {
    pub fn new(seq: Box<dyn Sequence>, match_name: String) -> Self {
        Self { seq, match_name }
    }
}

pub struct OneOrMoreSeq {
    pub match_name: String,
    pub seq: Box<dyn Sequence>,
}

impl Sequence for OneOrMoreSeq {
    fn match_tokens<'a>(&'a self, tokens: &[Token<'a>], refs: &'a RefMap) -> Option<TokenMatch> {
        let mut match_index = 0;
        let mut children = vec![];
        while let Some(matched) = self.seq.match_tokens(&tokens[match_index..], refs) {
            children.push((self.match_name.clone(), matched.new_token));
            match_index += matched.len;
        }
        if match_index > 0 {
            Some(TokenMatch {
                len: match_index,
                new_token: Token {
                    source: tokens.get(0).map_or_else(|| "", |t| t.source),
                    data: TokenData::Branch(children),
                },
            })
        } else {
            None
        }
    }
}

impl OneOrMoreSeq {
    pub fn new(seq: Box<dyn Sequence>, match_name: String) -> Self {
        Self { seq, match_name }
    }
}

pub struct ChooseSeq {
    pub seqs: Vec<(Box<dyn Sequence>, String)>,
}

impl Sequence for ChooseSeq {
    fn match_tokens<'a>(&'a self, tokens: &[Token<'a>], refs: &'a RefMap) -> Option<TokenMatch> {
        for (seq, name) in &self.seqs {
            if let Some(matched) = seq.match_tokens(tokens, refs) {
                return Some(TokenMatch {
                    len: matched.len,
                    new_token: Token {
                        source: matched.new_token.source,
                        data: TokenData::Branch(vec![(name.to_string(), matched.new_token)]),
                    },
                });
            }
        }
        None
    }
}

impl ChooseSeq {
    pub fn new(seqs: Vec<(Box<dyn Sequence>, String)>) -> Self {
        Self { seqs }
    }

    pub fn from_chars(chars: &str) -> Self {
        Self {
            seqs: chars
                .chars()
                .map(|c| {
                    (
                        Box::new(RawSeq::new(c.to_string())) as Box<dyn Sequence>,
                        c.to_string(),
                    )
                })
                .collect(),
        }
    }
}

pub struct RefSeq {
    pub name: String,
}

impl Sequence for RefSeq {
    fn match_tokens<'a>(&'a self, tokens: &[Token<'a>], refs: &'a RefMap) -> Option<TokenMatch> {
        refs.get(&self.name)
            .and_then(|s| s.match_tokens(tokens, refs))
    }
}

impl RefSeq {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Clone, PartialEq)]
pub struct AnySeq {}

impl Sequence for AnySeq {
    fn match_tokens<'a>(&'a self, tokens: &[Token<'a>], _: &'a RefMap) -> Option<TokenMatch> {
        if !tokens.is_empty() {
            Some(TokenMatch {
                len: 1,
                new_token: tokens[0].clone(),
            })
        } else {
            None
        }
    }
}

impl AnySeq {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for AnySeq {
    fn default() -> Self {
        Self::new()
    }
}

pub struct WhereSeq {
    pub predicate: Box<dyn Fn(&Token<'_>) -> bool>,
}

impl Sequence for WhereSeq {
    fn match_tokens<'a>(&'a self, tokens: &[Token<'a>], _: &'a RefMap) -> Option<TokenMatch> {
        tokens.first().and_then(move |t| {
            if (self.predicate)(t) {
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

impl WhereSeq {
    pub fn new(predicate: Box<dyn Fn(&Token<'_>) -> bool>) -> Self {
        Self { predicate }
    }
}

#[derive(Clone, PartialEq)]
pub struct RangeSeq {
    pub start: u32,
    pub end: u32,
}

impl Sequence for RangeSeq {
    fn match_tokens<'a>(&'a self, tokens: &[Token<'a>], _: &'a RefMap) -> Option<TokenMatch> {
        tokens.first().and_then(move |t| {
            let first_char = t.content().chars().next().unwrap() as u32;
            if (self.start..=self.end).contains(&first_char) {
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

impl RangeSeq {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }
}

#[derive(Clone, PartialEq)]
pub struct WhitespaceSeq {}

impl Sequence for WhitespaceSeq {
    fn match_tokens<'a>(&'a self, tokens: &[Token<'a>], _: &'a RefMap) -> Option<TokenMatch> {
        let mut match_index = 0;
        let mut children = vec![];
        while !tokens[match_index..].is_empty()
            && tokens[match_index]
                .content()
                .chars()
                .all(|c| c.is_whitespace())
        {
            children.push(("".to_string(), tokens[0].clone()));
            match_index += 1;
        }
        Some(TokenMatch {
            len: match_index,
            new_token: Token {
                source: tokens.get(0).map_or_else(|| "", |t| t.source),
                data: TokenData::Branch(children),
            },
        })
    }
}

impl WhitespaceSeq {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for WhitespaceSeq {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, PartialEq)]
pub struct NilSeq {}

impl Sequence for NilSeq {
    fn match_tokens<'a>(&'a self, _: &[Token<'a>], _: &'a RefMap) -> Option<TokenMatch> {
        Some(TokenMatch {
            len: 0,
            new_token: Token {
                source: "",
                data: TokenData::Leaf(0..0),
            },
        })
    }
}

impl NilSeq {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for NilSeq {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ExceptSeq {
    pub except: Box<dyn Sequence>,
}

impl Sequence for ExceptSeq {
    fn match_tokens<'a>(&'a self, tokens: &[Token<'a>], refs: &'a RefMap) -> Option<TokenMatch> {
        let Some(first_token) = tokens.get(0) else {
            return None;
        };

        if self.except.match_tokens(tokens, refs).is_some() {
            None
        } else {
            Some(TokenMatch {
                len: 1,
                new_token: first_token.clone(),
            })
        }
    }
}

impl ExceptSeq {
    pub fn new(except: Box<dyn Sequence>) -> Self {
        Self { except }
    }
}
