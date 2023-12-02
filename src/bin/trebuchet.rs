use regex::{Match, Regex};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let res1 = part_one()?;
    let res2 = part_two()?;
    println!("Part 1: {res1}, Part 2: {res2}");
    Ok(())
}

fn part_one() -> std::io::Result<usize> {
    let matcher = Regex::new(r"(\d).*(\d)|(\d)").unwrap();
    find_sum(matcher, |reg_match| Some(reg_match?.as_str()))
}

fn part_two() -> std::io::Result<usize>{
    let matcher = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine).*(\d|one|two|three|four|five|six|seven|eight|nine)|(\d|one|two|three|four|five|six|seven|eight|nine)")
        .unwrap();
    find_sum(matcher, |reg_match| {
        Some(match reg_match?.as_str() {
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
            "nine" => "9",
            rest => rest,
        })
    })
}

fn find_sum<F>(matcher: Regex, match_converter: F) -> std::io::Result<usize>
where
    F: Fn(Option<Match<'_>>) -> Option<&str>,
{
    let input_file = BufReader::new(File::open("./resources/day1.txt")?);
    let res: usize = input_file
        .lines()
        .map(|line| {
            let num: usize = matcher
                .captures(&line.unwrap())
                .unwrap()
                .iter()
                .skip(1)
                .filter_map(&match_converter)
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
    Ok(res)
}
