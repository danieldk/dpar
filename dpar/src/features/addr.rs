use std::borrow::Cow;

use conllx::Features;

use system::ParserState;

/// Source of tokens in the parser state.
///
/// For example, `Stack(1)` refers to the second token on the stack.
/// `LDep`/`RDep` are used to address from the left-most/right-most
/// dependency.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Source {
    /// The parse stack. In a typical transition system, these are
    /// tokens undergoing active processing.
    Stack(usize),

    /// The parse buffer. In a typical transition system, the buffer
    /// consists of unprocessed tokens.
    Buffer(usize),

    /// The n-th leftmost dependency.
    LDep(usize),

    /// The n-th rightmost dependency.
    RDep(usize),
}

/// Layer in the parser state.
///
/// In addressing the parser state, the layer is used to request
/// a specific type of information of a token, such as its part
/// of speech tag or the relation to its head (if available).
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Layer {
    /// The token (string).
    Token,

    /// Part-of-speech tag.
    Tag,

    /// Dependency relation to the token's head (if already attached).
    DepRel,

    /// Feature, the string argument should specify the feature name.
    Feature(String),
}

/// An `AddressedValue` represents a value in the parser state.
///
/// To create features, we need to address value in the parser
/// state. The purpose of AddressedValue is two-fold:
///
/// 1. As an address specifier;
/// 2. to obtain the value at that address in the current parser state.
///
/// If we want to get the part-of-speech tag of the leftmost dependent
/// of the first token on the stack, we could create the following
/// instance:
///
/// ```
/// use dpar::features::addr::{AddressedValue, Layer, Source};
///
/// let addr = AddressedValue {
///   address: vec![Source::Stack(0), Source::LDep(0)],
///   layer: Layer::Tag,
/// };
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AddressedValue {
    pub address: Vec<Source>,
    pub layer: Layer,
}

impl AddressedValue {
    /// Get the value of the address in the given parser state.
    pub fn get<'a>(&self, state: &'a ParserState) -> Option<Cow<'a, str>> {
        let mut token = 0;
        for (idx, source) in self.address.iter().enumerate() {
            if let Some(next_token) = self.resolve_address(state, *source, idx, token) {
                token = next_token;
            } else {
                return None;
            }
        }

        self.resolve_value(state, token)
    }

    fn resolve_address(
        &self,
        state: &ParserState,
        source: Source,
        idx: usize,
        token: usize,
    ) -> Option<usize> {
        match source {
            Source::Stack(n) => {
                // Stack(0) corresponds to stack.len() - 1.
                let n = state.stack().len().checked_sub(n + 1)?;
                state.stack().get(n).cloned()
            }
            Source::Buffer(n) => state.buffer().get(n).cloned(),
            Source::LDep(n) => {
                assert!(idx != 0, "LDEP cannot be the initial address component");
                state.leftmost_dependent(token, n)
            }
            Source::RDep(n) => {
                assert!(idx != 0, "RDEP cannot be the initial address component");
                state.rightmost_dependent(token, n)
            }
        }
    }

    fn resolve_value<'a>(&self, state: &'a ParserState, token: usize) -> Option<Cow<'a, str>> {
        match self.layer {
            // Note: indexing is used here rather than get(), accessing a non-existing
            // token is a bug. So we want things to burst in flames.
            Layer::Token => Some(Cow::Borrowed(state.tokens()[token])),
            Layer::Tag => Some(Cow::Borrowed(state.tags()[token])),
            Layer::DepRel => state
                .head(token)
                .map(|dep| Cow::Borrowed(dep.relation.as_str())),
            Layer::Feature(ref feat) => {
                let feature_map = state.features()[token].map(Features::as_map)?;

                // Return None when the feature or the feature value is absent.
                feature_map
                    .get(feat)
                    .map(Option::as_ref)
                    .and_then(|x| x) // Option<Option<T>> -> Option<T>
                    .map(String::as_str)
                    .map(Cow::Borrowed)
            }
        }
    }
}
