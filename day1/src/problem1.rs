pub mod problem1 {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    const STARTING_FREQ: i32 = 0;

    pub fn solve() {
        let freq_file = "frequencies.txt";
        let f = File::open(freq_file).expect("Unable to open file");
        let f = BufReader::new(f);
        let mut current_freq: i32 = STARTING_FREQ;

        for line in f.lines() {
            let line  = String::from(line.unwrap());
            let parsed_freq = parse_line(&line);
            match parsed_freq.sign {
                Sign::Plus => current_freq = current_freq + parsed_freq.value,
                Sign::Minus => current_freq = current_freq - parsed_freq.value,
            }
        }
        println!("Final frequency is {}", current_freq);

    }

    fn parse_line(line: &str) -> Freq {
        let sign = String::from(&line[..1]);
        let value: i32 = String::from(&line[1..]).parse().unwrap();
        match sign.as_ref() {
            "+" => Freq { sign: Sign::Plus, value: value },
            "-" => Freq { sign: Sign::Minus, value: value },
            _ => panic!("Could not parse sign"),
        }
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
}
