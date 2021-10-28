use std::io::{BufRead, BufReader};

pub fn find_matches(file: &std::fs::File, pattern: &str) -> Vec<usize> {
    let reader = BufReader::new(file);
    let mut matches = Vec::new();
    for (index, line) in reader.lines().enumerate() {
        if line.unwrap().contains(&pattern) {
            matches.push(index);
        }
    }
    matches
}
