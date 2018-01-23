use std::fs::File;
use std::io::BufReader;

use conllx::Reader;

use guide::Guide;
use parser::{GreedyParser, Parse};
use system::*;
use systems::arc_eager::ArcEagerOracle;
use systems::arc_standard::ArcStandardOracle;
use systems::stack_projective::StackProjectiveOracle;
use systems::stack_swap::StackSwapOracle;

static PROJECTIVE_DATA: &'static str = "testdata/cdb-test.conll";

static NON_PROJECTIVE_DATA: &'static str = "testdata/cdb-test-np.conll";

fn test_system<G>(oracle_constructor: fn(&DependencySet) -> G, data: &str)
where
    G: Guide,
{
    let f = File::open(data).unwrap();
    let reader = Reader::new(BufReader::new(f));

    for sentence in reader {
        let sentence = sentence.unwrap();
        let dependencies = sentence_to_dependencies(&sentence).unwrap();

        let oracle = oracle_constructor(&dependencies);
        let mut parser = GreedyParser::new(oracle);

        let result = parser.parse(&sentence).unwrap();

        assert_eq!(&dependencies, &result);
    }
}

#[test]
fn test_arc_eager() {
    test_system(ArcEagerOracle::new, PROJECTIVE_DATA);
}

#[test]
fn test_arc_standard() {
    test_system(ArcStandardOracle::new, PROJECTIVE_DATA);
}


#[test]
fn test_stack_projective() {
    test_system(StackProjectiveOracle::new, PROJECTIVE_DATA);
}


#[test]
fn test_stack_swap() {
    test_system(StackSwapOracle::new, NON_PROJECTIVE_DATA);
}
