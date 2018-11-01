use features::addr::{AddressedValue, Layer, Source};

use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::Parser;

#[derive(Parser)]
#[grammar = "features/addr.pest"]
pub struct AddressedValueParser;

pub fn parse_addressed_values(data: &str) -> Result<Vec<AddressedValue>, Error<Rule>> {
    let file = AddressedValueParser::parse(Rule::file, &data)?
        .next()
        .unwrap();

    let mut addr_values = Vec::new();

    for addr_value_pair in file.into_inner() {
        match addr_value_pair.as_rule() {
            Rule::addr_value => {
                addr_values.push(process_addressed_value(addr_value_pair.into_inner()))
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Ok(addr_values)
}

fn process_addressed_value(mut pairs: Pairs<Rule>) -> AddressedValue {
    let address_pair = pairs
        .next()
        .expect("Address components missing")
        .into_inner();
    let layer_pair = pairs
        .next()
        .expect("Layer missing")
        .into_inner()
        .next()
        .expect("Layer missing");

    let address = process_addresses(address_pair);
    let layer = process_layer(layer_pair);

    AddressedValue { address, layer }
}

fn process_address(pair: Pair<Rule>) -> Source {
    let rule = pair.as_rule();

    let source_idx = pair
        .into_inner()
        .next()
        .expect("Missing source index")
        .as_str()
        .parse()
        .expect("Invalid source index");

    match rule {
        Rule::stack => Source::Stack(source_idx),
        Rule::buffer => Source::Buffer(source_idx),
        Rule::ldep => Source::LDep(source_idx),
        Rule::rdep => Source::RDep(source_idx),
        _ => unreachable!(),
    }
}

fn process_addresses(pairs: Pairs<Rule>) -> Vec<Source> {
    let mut address = Vec::new();

    for source in pairs {
        match source.as_rule() {
            Rule::initial_source | Rule::dep_source => {
                address.push(process_address(
                    source.into_inner().next().expect("Cannot get component"),
                ));
            }
            _ => unreachable!(),
        }
    }

    address
}

fn process_layer(pair: Pair<Rule>) -> Layer {
    match pair.as_rule() {
        Rule::char_layer => {
            let mut pairs = pair.into_inner();
            let prefix = pairs
                .next()
                .expect("Missing prefix")
                .as_str()
                .parse()
                .expect("Invalid prefix length");
            let suffix = pairs
                .next()
                .expect("Missing prefix")
                .as_str()
                .parse()
                .expect("Invalid suffix length");
            Layer::Char(prefix, suffix)
        }
        Rule::deprel_layer => Layer::DepRel,
        Rule::feature_layer => {
            let name = pair.into_inner().next().expect("Missing feature name");
            Layer::Feature(name.as_str().to_string())
        }
        Rule::tag_layer => Layer::Tag,
        Rule::token_layer => Layer::Token,
        _ => unreachable!(),
    }
}
