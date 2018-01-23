use std::borrow::Cow;

use conllx::{Features, Sentence, TokenBuilder};

use features::addr::{AddressedValue, Layer, Source};
use system::{ParserState, Transition};
use systems::stack_projective::StackProjectiveTransition;

lazy_static! {
    static ref CHAR_TEST_SENTENCE: Sentence = Sentence::new(vec![TokenBuilder::new("Orientierungswoche").token()]);
    static ref CHAR_TEST_PARSER_STATE: ParserState<'static> = ParserState::new(&CHAR_TEST_SENTENCE);

    static ref SHORT_CHAR_TEST_SENTENCE: Sentence = Sentence::new(vec![TokenBuilder::new("zu").token()]);
    static ref SHORT_CHAR_TEST_PARSER_STATE: ParserState<'static> = ParserState::new(&SHORT_CHAR_TEST_SENTENCE);

    static ref CHAR_TEST_ADDR: AddressedValue = AddressedValue {
        address: vec![Source::Buffer(0)],
        layer: Layer::Char(3, 4)
    };

    static ref THREE_TOKEN_SENTENCE: Sentence = Sentence::new(vec![
        TokenBuilder::new("a").features(Features::from_string("a:x|b")).token(),
        TokenBuilder::new("b").features(Features::from_string("c:y|d")).token(),
        TokenBuilder::new("c").token()
    ]);
}

#[test]
fn test_char() {
    assert_eq!(
        Some(Cow::Borrowed("Orioche")),
        CHAR_TEST_ADDR.get(&CHAR_TEST_PARSER_STATE)
    );
    assert_eq!(
        Some(Cow::Borrowed("zu\0\0\0zu")),
        CHAR_TEST_ADDR.get(&SHORT_CHAR_TEST_PARSER_STATE)
    );
}

#[test]
fn test_features() {
    let buffer0a = AddressedValue {
        address: vec![Source::Buffer(0)],
        layer: Layer::Feature("a".to_owned()),
    };

    let buffer0b = AddressedValue {
        address: vec![Source::Buffer(0)],
        layer: Layer::Feature("b".to_owned()),
    };

    let buffer2e = AddressedValue {
        address: vec![Source::Buffer(0)],
        layer: Layer::Feature("e".to_owned()),
    };

    let state = ParserState::new(&THREE_TOKEN_SENTENCE);

    assert_eq!(Some(Cow::Borrowed("x")), buffer0a.get(&state));
    assert_eq!(None, buffer0b.get(&state));
    assert_eq!(None, buffer2e.get(&state));
}

#[test]
fn test_ldep_rdep() {
    let ldep0 = AddressedValue {
        address: vec![Source::Stack(0), Source::LDep(0)],
        layer: Layer::DepRel,
    };

    let ldep1 = AddressedValue {
        address: vec![Source::Stack(0), Source::LDep(1)],
        layer: Layer::DepRel,
    };

    let rdep0 = AddressedValue {
        address: vec![Source::Stack(0), Source::RDep(0)],
        layer: Layer::DepRel,
    };

    let rdep1 = AddressedValue {
        address: vec![Source::Stack(0), Source::RDep(1)],
        layer: Layer::DepRel,
    };


    let mut state = ParserState::new(&THREE_TOKEN_SENTENCE);
    StackProjectiveTransition::Shift.apply(&mut state);
    StackProjectiveTransition::Shift.apply(&mut state);
    StackProjectiveTransition::LeftArc("Foo".to_owned()).apply(&mut state);
    StackProjectiveTransition::Shift.apply(&mut state);
    StackProjectiveTransition::RightArc("Bar".to_owned()).apply(&mut state);

    assert_eq!(Some(Cow::Borrowed("Foo")), ldep0.get(&state));
    assert_eq!(Some(Cow::Borrowed("Bar")), ldep1.get(&state));
    assert_eq!(Some(Cow::Borrowed("Bar")), rdep0.get(&state));
    assert_eq!(Some(Cow::Borrowed("Foo")), rdep1.get(&state));
}
