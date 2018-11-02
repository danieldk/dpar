use std::collections::HashSet;

use conllx::{Features, Sentence, Token};

use system::{Dependency, DependencySet};

#[derive(Debug)]
pub struct ParserState<'a> {
    tokens: Vec<&'a str>,
    tags: Vec<&'a str>,
    features: Vec<Option<&'a Features>>,
    stack: Vec<usize>,
    buffer: Vec<usize>,
    token_heads: Vec<Option<Dependency>>,
    head_deps: Vec<Vec<usize>>,
}

impl<'a> ParserState<'a> {
    pub fn new(sentence: &Sentence) -> ParserState {
        let n_tokens = sentence.len() + 1;

        let buffer = (1..n_tokens).collect();
        let stack = vec![0];

        let mut tokens = Vec::with_capacity(n_tokens);
        tokens.push("ROOT");
        tokens.extend(sentence.iter().map(Token::form));

        let mut tags = Vec::with_capacity(n_tokens);
        tags.push("ROOT");
        tags.extend(sentence.iter().map(Token::pos).map(|f| f.unwrap_or("_")));

        let mut features = Vec::with_capacity(n_tokens);
        features.push(None);
        features.extend(sentence.iter().map(Token::features));

        return ParserState {
            tokens: tokens,
            tags: tags,
            features: features,
            stack: stack,
            buffer: buffer,
            token_heads: vec![None; n_tokens],
            head_deps: vec![Vec::new(); n_tokens],
        };
    }

    pub fn add_dependency(&mut self, d: Dependency) {
        let head_idx = d.head;
        let dep_idx = d.dependent;

        if let Err(pos) = self.head_deps[head_idx].binary_search(&dep_idx) {
            self.head_deps[head_idx].insert(pos, dep_idx)
        };

        self.token_heads[dep_idx] = Some(d);
    }

    pub fn dependencies(&self) -> DependencySet {
        let mut deps = HashSet::new();

        for dep in &self.token_heads {
            if let Some(dep) = dep.as_ref() {
                deps.insert(dep.clone());
            }
        }

        deps
    }

    pub fn features(&self) -> &[Option<&Features>] {
        &self.features
    }

    pub fn head(&self, token: usize) -> Option<&Dependency> {
        self.token_heads[token].as_ref()
    }

    pub fn leftmost_dependent(&self, head: usize, idx: usize) -> Option<usize> {
        self.head_deps[head].get(idx).cloned()
    }

    pub fn rightmost_dependent(&self, head: usize, idx: usize) -> Option<usize> {
        let deps = &self.head_deps[head];
        let idx = deps.len().checked_sub(idx + 1)?;
        deps.get(idx).cloned()
    }

    pub fn stack(&self) -> &Vec<usize> {
        &self.stack
    }

    pub fn stack_mut(&mut self) -> &mut Vec<usize> {
        &mut self.stack
    }

    pub fn buffer(&self) -> &Vec<usize> {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut Vec<usize> {
        &mut self.buffer
    }

    pub fn tags(&self) -> &[&str] {
        self.tags.as_slice()
    }

    pub fn tokens(&self) -> &[&str] {
        self.tokens.as_slice()
    }
}
