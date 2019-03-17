use std::collections::HashMap;

use features::addr::{AddressedValue, Layer, Source};
use features::parse_addr::parse_addressed_values;

static CORRECT_STRING1: &'static str = "[STACK 0] TOKEN";
static CORRECT_STRING2: &'static str = "[BUFFER 1] TAG";
static CORRECT_STRING3: &'static str = "[STACK 0, LDEP 0] DEPREL";
static CORRECT_STRING4: &'static str = "[STACK 0, LDEP 0] DEPREL [STACK 0, RDEP 0] DEPREL";
static CORRECT_STRING5: &'static str = "[STACK 0] FEATURE num";
static CORRECT_STRING6: &'static str =
    "[STACK 0,\n  LDEP 0]\n  DEPREL\n  [STACK 0,\n RDEP 0] DEPREL";

lazy_static! {
    static ref CORRECT1: Vec<AddressedValue> = vec![AddressedValue {
        address: vec![Source::Stack(0)],
        layer: Layer::Token,
    }];
    static ref CORRECT2: Vec<AddressedValue> = vec![AddressedValue {
        address: vec![Source::Buffer(1)],
        layer: Layer::Tag,
    }];
    static ref CORRECT3: Vec<AddressedValue> = vec![AddressedValue {
        address: vec![Source::Stack(0), Source::LDep(0)],
        layer: Layer::DepRel,
    }];
    static ref CORRECT4: Vec<AddressedValue> = vec![
        AddressedValue {
            address: vec![Source::Stack(0), Source::LDep(0)],
            layer: Layer::DepRel,
        },
        AddressedValue {
            address: vec![Source::Stack(0), Source::RDep(0)],
            layer: Layer::DepRel,
        },
    ];
    static ref CORRECT5: Vec<AddressedValue> = vec![AddressedValue {
        address: vec![Source::Stack(0)],
        layer: Layer::Feature("num".to_owned()),
    }];
    static ref CORRECT_CASES: HashMap<&'static str, Vec<AddressedValue>> = hashmap! {
        CORRECT_STRING1 => CORRECT1.clone(),
        CORRECT_STRING2 => CORRECT2.clone(),
        CORRECT_STRING3 => CORRECT3.clone(),
        CORRECT_STRING4 => CORRECT4.clone(),
        CORRECT_STRING5 => CORRECT5.clone(),
        CORRECT_STRING6 => CORRECT4.clone(),
    };
    static ref INCORRECT_CASES: Vec<&'static str> = vec![
        "[] TOKEN",
        "[STACK 0,] TOKEN",
        "[DRAWER 0] TOKEN",
        "[STACK 0] CANDY",
        ",[STACK 0] TOKEN",
        "STACK 0 TOKEN",
        "[STACK 0 TOKEN",
        "STACK 0] TOKEN",
        "[STACK 0]",
        "[STACK 0] TOKEN TAG",
        "[STACK 0] TOKEN [STACK 1]",
        "[LDEP 0] TOKEN",
        "[RDEP 0] TOKEN",
        "[STACK 0] TOKEN num",
        "[STACK 0] TAG num",
        "[STACK 0] DEPREL num",
        "[STACK 0, STACK 0] TOKEN",
        "[BUFFER 0, BUFFER 0] TOKEN",
        "[STACK 0, LDEP 0, BUFFER 0] TOKEN",
        "[STACK 0, LDEP 0, BUFFER 0] TOKEN",
        "[STACK\n0, LDEP 0] DEPREL",
        "[STACK 0, LDEP 0] FEATURE\ntf",
    ];
}

#[test]
fn test_correct() {
    for (data, correct) in CORRECT_CASES.iter() {
        let result = parse_addressed_values(&data).unwrap();
        assert_eq!(*correct, result);
    }
}

#[test]
fn incorrect_cases() {
    for data in INCORRECT_CASES.iter() {
        assert!(parse_addressed_values(&data).is_err());
    }
}
