use core::ops::Range;

pub struct Token<'a> {
    pub source: &'a str,
    pub content_range: Range<usize>,
    pub children: Vec<(String, &'a Token<'a>)>,
}

impl<'a> Token<'a> {
    pub fn content(&self) -> &str {
        &self.source[self.content_range.clone()]
    }

    pub fn get_children_iter<'b>(
        &'b self,
        key: &'b str,
    ) -> impl Iterator<Item = &&'a Token<'a>> + 'b {
        self.children
            .iter()
            .filter(move |(k, _)| k == key)
            .map(|(_, v)| v)
    }

    pub fn get_children<'b>(&'b self, key: &'b str) -> Vec<&&'a Token<'a>> {
        self.get_children_iter(key).collect()
    }
}
