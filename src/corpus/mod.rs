use crate::token::*;

pub struct Corpus<'a> {
    pub tokens: Vec<Token<'a>>,
}

impl<'a> Corpus<'a> {
    pub fn make(text: &'a str) -> Self {
        Corpus {
            tokens: (0..text.len())
                .map(|i| Token {
                    source: text,
                    data: TokenData::Leaf(i..i + 1),
                })
                .collect(),
        }
    }

    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Corpus { tokens }
    }
}
