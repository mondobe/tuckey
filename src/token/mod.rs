use core::ops::Range;

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub source: &'a str,
    pub data: TokenData<'a>,
}

#[derive(Debug, Clone)]
pub enum TokenData<'a> {
    Leaf(Range<usize>),
    Branch(Vec<(String, Token<'a>)>),
}

impl<'a> Token<'a> {
    pub fn content_range(&self) -> Range<usize> {
        match &self.data {
            TokenData::Leaf(range) => range.clone(),
            TokenData::Branch(children) => {
                children[0].1.content_range().start..children.last().unwrap().1.content_range().end
            }
        }
    }

    pub fn content(&self) -> &str {
        &self.source[self.content_range()]
    }

    pub fn get_children_iter(
        &'a self,
        key: &'a str,
    ) -> Option<impl Iterator<Item = &'a Token<'a>>> {
        match &self.data {
            TokenData::Leaf(_) => None,
            TokenData::Branch(children) => Some(
                children
                    .iter()
                    .filter(move |(k, _)| k == key)
                    .map(move |(_, v)| v),
            ),
        }
    }

    pub fn get_children(&'a self, key: &'a str) -> Option<Vec<&'a Token<'a>>> {
        self.get_children_iter(key).map(|i| i.collect())
    }
}
