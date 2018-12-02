use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    let input_filename = "inputs.txt";
    let f = File::open(input_filename).expect("Unable to open file");
    let f = BufReader::new(f);
    let mut all_lines: Vec<String> = Vec::new();
    for line in f.lines() {
        let line = line.unwrap();
        all_lines.push(line);
    }

    let inner_lines = all_lines.clone();
    for line in all_lines.iter() {
        for other_line in inner_lines.iter() {
            let diff = difference(line, other_line);
            if diff.len() == 1 {
                println!("{}", line);
                println!("{}", other_line);
                println!("{}", diff);
                println!("Found it! {}", intersection(line, other_line));
                return;
            }
        }
    }
}

fn difference(str1: &str, str2: &str) -> String {
    let mut diff = String::new();
    for (idx, chr) in str1.char_indices() {
        let mut str2_chars = str2.chars();
        if str2_chars.nth(idx).unwrap() != chr {
            diff.push(chr);
        }
    }
    return diff;
}

fn intersection(str1: &str, str2: &str) -> String {
    let mut intersection = String::new();
    for (idx, chr) in str1.char_indices() {
        let mut str2_chars = str2.chars();
        if str2_chars.nth(idx).unwrap() == chr {
            intersection.push(chr);
        }
    }
    return intersection;

}

#[test]
fn test_difference() {
    assert_eq!("b", difference("but", "nut"));
    assert_eq!("a", difference("smart", "smort"));
    assert_eq!("hey", difference("chewy", "cxxwx"));
}

#[test]
fn test_intersection() {
    assert_eq!("ut", intersection("but", "nut"));
}
