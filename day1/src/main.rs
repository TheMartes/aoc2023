use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let lines = lines_from_file("input_p1.txt");
    let mut sum = 0;

    for line in lines {
        let mut nums: Vec<u32> = Vec::new();

        for c in line.chars() {
            if c.is_numeric() {
                let to_append = c.to_digit(10).unwrap_or(0);
                nums.push(to_append);
            }
        }
        let ln = nums.len();
        if ln == 1 {
            // bcs its first and las therefore its * 11
            sum += nums[0] * 11;
            continue;
        }
        if ln == 0 {
            continue;
        }

        // fuck me
        let cnct = nums[0].to_string() + &nums[ln - 1].to_string();
        match cnct.parse::<u32>() {
            Ok(number) => {
                sum += number;
            }
            Err(_) => {
                println!("Failed to parse the number!"); // Handle the case where parsing fails
            }
        }
    }

    println!("part one: {}", sum)
}

fn part_two() {
    let lines = lines_from_file("input_p1.txt");

    let lexicon: [String; 9] = [
        String::from("one"),
        String::from("two"),
        String::from("three"),
        String::from("four"),
        String::from("five"),
        String::from("six"),
        String::from("seven"),
        String::from("eight"),
        String::from("nine")
    ];

    let mut mapper: HashMap<String, String> = HashMap::new();
    mapper.insert("one".to_string(), String::from("1"));
    mapper.insert("two".to_string(), String::from("2"));
    mapper.insert("three".to_string(), String::from("3"));
    mapper.insert("four".to_string(), String::from("4"));
    mapper.insert("five".to_string(), String::from("5"));
    mapper.insert("six".to_string(), String::from("6"));
    mapper.insert("seven".to_string(), String::from("7"));
    mapper.insert("eight".to_string(), String::from("8"));
    mapper.insert("nine".to_string(), String::from("9"));

    let mut sum = 0;

    for line in lines {
        let mut hm: HashMap<usize, String> = HashMap::new();

        for (i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                hm.insert(i, String::from(c));
            }
        }

        for n in &lexicon {
            for (index, _) in line.match_indices(n) {
                let rn = mapper.get(n).unwrap();
                if rn != "" {
                    hm.insert(index, String::from(rn));
                }
            }
        }

        let mut keys: Vec<&usize> = hm.keys().collect();
        keys.sort();

        let first_number = hm.get(
            keys.iter().min().unwrap()
        ).unwrap();
        let last_number = hm.get(
            keys.iter().max().unwrap()
        ).unwrap();

        let concatenated = first_number.to_string() + &*last_number.to_string();

        match concatenated.parse::<u32>() {
            Ok(number) => {
                sum += number;
            }
            Err(_) => {
                println!("Failed to parse the number!"); // Handle the case where parsing fails
            }
        }
    }

    println!("part two: {}", sum)
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
