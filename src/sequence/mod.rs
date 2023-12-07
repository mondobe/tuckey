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
}

impl Sequence for OptSeq {
    fn match_tokens<'a>(&'a self, tokens: &[Token<'a>], refs: &'a RefMap) -> Option<TokenMatch> {
        match self.seq.match_tokens(tokens, refs) {
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
    pub fn new(seq: Box<dyn Sequence>) -> Self {
        Self { seq }
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
    pub match_name: Option<String>,
    pub seq: Box<dyn Sequence>,
}

impl Sequence for OneOrMoreSeq {
    fn match_tokens<'a>(&'a self, tokens: &[Token<'a>], refs: &'a RefMap) -> Option<TokenMatch> {
        let mut match_index = 0;
        let mut children = vec![];
        while let Some(matched) = self.seq.match_tokens(&tokens[match_index..], refs) {
            if let Some(name) = &self.match_name {
                children.push((name.to_string(), matched.new_token));
            }
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
    pub fn new(seq: Box<dyn Sequence>, match_name: Option<String>) -> Self {
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
