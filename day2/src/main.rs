use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

struct HighestOccurance {
    red: i32,
    green: i32,
    blue: i32,
}

fn main() {
    let lines = lines_from_file("input.txt");
    let mut result = 0;
    let mut power_sum = 0;

    for line in lines {
        let game_meta: Vec<&str> = line.split(": ").collect();
        let game_info_split: Vec<&str> = game_meta[0].split(" ").collect(); // ["Game", "1"]
        let game_id = game_info_split[1]; // ["1"]
        let rounds: Vec<&str> = game_meta[1].split("; ").collect();
        let mut ho = HighestOccurance{ red: 0, green: 0, blue: 0 };

        let mut game_possible = true;

        for round in rounds {
            let cube_combinations: Vec<&str> = round.split(", ").collect();

            for combination in cube_combinations {
                let split_string: Vec<&str> = combination.split(" ").collect();
                let count_raw = split_string[0];
                let color = split_string[1];
                let count = count_raw.parse::<i32>().unwrap();

                match color {
                    "red" => {
                        if count > 12 { game_possible = false }
                        if count > ho.red {
                            ho.red = count
                        }
                    },
                    "green" => {
                        if count > 13 { game_possible = false }
                        if count > ho.green {
                            ho.green = count
                        }
                    },
                    "blue" => {
                        if count > 14 { game_possible = false }
                        if count > ho.blue {
                            ho.blue = count
                        }
                    },
                    _ => println!("Can't match cube combination")
                }
            }
        }

        if game_possible {
            result += game_id.parse::<i32>().unwrap()
        }

        power_sum += (ho.red * ho.blue * ho.green)
    }

    println!("result part one: {}", result);
    println!("result part two: {}", power_sum);
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
