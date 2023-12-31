use core::ops::Range;
use std::fmt::Debug;

#[derive(Clone)]
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
    pub fn content_range(&self) -> Option<Range<usize>> {
        match &self.data {
            TokenData::Leaf(range) => Some(range.clone()),
            TokenData::Branch(children) if children.is_empty() => None,
            TokenData::Branch(children) => {
                let iter: Vec<Range<usize>> = children
                    .iter()
                    .filter_map(|s| s.1.content_range())
                    .collect();
                iter.first()
                    .map(|first| first.start..iter.last().unwrap().end)
            }
        }
    }

    pub fn content(&self) -> &str {
        self.content_range().map_or_else(|| "", |r| &self.source[r])
    }

    pub fn get_children(&'a self, key: &'a str) -> Vec<Token<'a>> {
        match &self.data {
            TokenData::Leaf(_) => vec![],
            TokenData::Branch(children) => children
                .iter()
                .filter(|i| i.0 == key)
                .map(|i| i.1.clone())
                .collect(),
        }
    }

    pub fn get_first_child(&'a self, key: &'a str) -> Option<Token<'a>> {
        self.get_children(key).get(0).cloned()
    }

    pub fn graph(&self) -> String {
        "\n".to_string() + &self.graph_depth(0)
    }

    fn graph_depth(&self, depth: usize) -> String {
        let mut graph = String::new();
        for _ in 0..depth {
            graph += "\t";
        }
        match &self.data {
            TokenData::Leaf(_) => {
                graph += &format!("{:?}", self.content());
            }
            TokenData::Branch(children) => {
                graph += "{\n";
                for child in children {
                    if !child.0.is_empty() {
                        for _ in 0..depth + 1 {
                            graph += "\t";
                        }
                        graph += &format!("{0}:\n{1},\n", child.0, child.1.graph_depth(depth + 1));
                    } else {
                        graph += &format!("{},\n", child.1.graph_depth(depth + 1));
                    }
                }
                for _ in 0..depth {
                    graph += "\t";
                }
                graph += "}";
            }
        }
        graph
    }
}

impl Debug for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.graph())
    }
}
