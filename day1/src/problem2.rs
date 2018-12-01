use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

const STARTING_FREQ: i32 = 0;

pub fn solve() {
    let mut seen_frequencies: HashMap<i32, i32> = HashMap::new();
    let mut current_freq = STARTING_FREQ;
    seen_frequencies.insert(current_freq, 1);
    let freq_ring = populate_ring();
    let found = false;

    while !found {
        for freq in freq_ring.iter() {
            match freq.sign {
                Sign::Plus => current_freq = current_freq + freq.value,
                Sign::Minus => current_freq = current_freq - freq.value,
            }
            if seen_frequencies.contains_key(&current_freq) {
                println!("Found second value {}", current_freq);
                return;
            } else {
                seen_frequencies.insert(current_freq, 1);
            }
        }
    }
}

fn parse_line(line: &str) -> Freq {
    let sign = String::from(&line[..1]);
    let value: i32 = String::from(&line[1..]).parse().unwrap();
    match sign.as_ref() {
        "+" => Freq {
            sign: Sign::Plus,
            value: value,
        },
        "-" => Freq {
            sign: Sign::Minus,
            value: value,
        },
        _ => panic!("Could not parse sign"),
    }
}

fn populate_ring() -> VecDeque<Freq> {
    let freq_file = "frequencies.txt";
    let f = File::open(freq_file).expect("Unable to open file");
    let f = BufReader::new(f);
    let mut ring: VecDeque<Freq> = VecDeque::new();

    for line in f.lines() {
        let line = String::from(line.unwrap());
        let parsed_freq = parse_line(&line);
        ring.push_back(parsed_freq);
    }
    return ring;
}

#[derive(Debug)]
struct Freq {
    sign: Sign,
    value: i32,
}

#[derive(Debug)]
enum Sign {
    Plus,
    Minus,
}
