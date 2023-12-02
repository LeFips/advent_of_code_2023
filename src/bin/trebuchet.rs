use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    part_one()?;
    part_two()?;
    Ok(())
}

fn part_one() -> std::io::Result<()> {
    let input_file = BufReader::new(File::open("./resources/day1.txt")?);
    let matcher = Regex::new(r"(\d).*(\d)|(\d)").unwrap();
    let res: usize = input_file
        .lines()
        .map(|line| {
            let num: usize = matcher
                .captures(&line.unwrap())
                .unwrap()
                .iter()
                .skip(1)
                .filter_map(|d| Some(d?.as_str()))
                .collect::<String>()
                .parse()
                .unwrap();
            if num > 10 {
                num
            } else {
                num * 10 + num
            }
        })
        .sum();
    println!("{res}");
    Ok(())
}

fn part_two() -> std::io::Result<()> {
    let input_file = BufReader::new(File::open("./resources/day1.txt")?);
    let matcher = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine).*(\d|one|two|three|four|five|six|seven|eight|nine)|(\d|one|two|three|four|five|six|seven|eight|nine)")
        .unwrap();
    let res: usize = input_file
        .lines()
        .map(|line| {
            let num: usize = matcher
                .captures(&line.unwrap())
                .unwrap()
                .iter()
                .skip(1)
                .filter_map(|d| {
                    Some(match d?.as_str() {
                        "one" => "1",
                        "two" => "2",
                        "three" => "3",
                        "four" => "4",
                        "five" => "5",
                        "six" => "6",
                        "seven" => "7",
                        "eight" => "8",
                        "nine" => "9",
                        rest => rest
                    })
                })
                .collect::<String>()
                .parse()
                .unwrap();
            if num > 10 {
                num
            } else {
                num * 10 + num
            }
        })
        .sum();
    println!("{res}");
    Ok(())
}
