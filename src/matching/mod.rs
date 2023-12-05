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
                    content_range: i..i + 1,
                    children: vec![],
                })
                .collect(),
        }
    }

    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Corpus { tokens }
    }
}
