use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

pub fn solve() {
    let input_filename = "inputs.txt";
    let f = File::open(input_filename).expect("Unable to open file");
    let f = BufReader::new(f);
    let mut has_twos = 0;
    let mut has_threes = 0;

    for line in f.lines() {
        let line = line.unwrap();
        let occurrences = count_occurrences(&line);
        if has_two(&occurrences) {
            has_twos = has_twos + 1;
        }
        if has_three(&occurrences) {
            has_threes = has_threes + 1;
        }
    }
    println!("Final checksum is: {}", has_twos * has_threes);
}

fn count_occurrences(line: &str) -> HashMap<String, u32> {
    let mut occurrences = HashMap::new();
    for character in line.chars() {
        let str_char = character.to_string();
        match &occurrences.get(&str_char) {
            Some(&val) => occurrences.insert(str_char, val + 1),
            None => occurrences.insert(str_char, 1),
        };
    }
    return occurrences;
}

fn has_two(occ_map: &HashMap<String, u32>) -> bool {
    occ_map.iter().any(|(_k, v)| *v == 2)
}

fn has_three(occ_map: &HashMap<String, u32>) -> bool {
    occ_map.iter().any(|(_k, v)| *v == 3)
}
