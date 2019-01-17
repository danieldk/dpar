use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use failure::Error;

/// Read association strengths for dependency triples from a text file.
///
/// Such a text file consists of lines with the tab-separated format
///
/// ~~~text,no_run
/// [token+] [token+] [deprel+] association_strength
/// ~~~
pub fn associations_from_buf_read(
    f: File,
) -> Result<HashMap<(String, String, String), f32>, Error> {
    let mut association_strengths: HashMap<(String, String, String), f32> = HashMap::new();
    for l in BufReader::new(f).lines() {
        let l = l.unwrap();
        let line = l.split("\t").collect::<Vec<_>>();
        association_strengths.insert(
            (
                line[0].to_string(),
                line[1].to_string(),
                line[2].to_string(),
            ),
            line[3].parse::<f32>().unwrap(),
        );
    }
    Ok(association_strengths)
}
