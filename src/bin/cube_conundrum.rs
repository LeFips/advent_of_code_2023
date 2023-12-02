use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let res1 = part_one(12, 13, 14);
    let res2 = part_two();
    println!("Part 1: {res1}, Part 2: {res2}");
}

fn part_one(max_red: usize, max_green: usize, max_blue: usize) -> usize {
    let games = parse_input();
    games
        .enumerate()
        .filter(|(_, game)| {
            game.max_red <= max_red && game.max_green <= max_green && game.max_blue <= max_blue
        })
        .map(|(index, _)| index + 1)
        .sum()
}

fn part_two() -> usize {
    let games = parse_input();
    games
        .map(|game| game.max_red * game.max_green * game.max_blue)
        .sum()
}

fn parse_input() -> impl Iterator<Item = Game> {
    let input_file = BufReader::new(File::open("./resources/day2.txt").unwrap());
    input_file.lines().map(|line| {
        let line = line.unwrap();
        let game_details = &line[line.find(':').unwrap() + 1..];
        let mut game = Game::new();

        for round in game_details.split(';') {
            for draw in round.split(',') {
                let mut draw_details = draw.trim().split(' ');
                let count: usize = draw_details.next().unwrap().parse().unwrap();
                let color = draw_details.next().unwrap();

                match color {
                    "red" => game.max_red = max(game.max_red, count),
                    "green" => game.max_green = max(game.max_green, count),
                    "blue" => game.max_blue = max(game.max_blue, count),
                    _ => (),
                }
            }
        }
        game
    })
}

struct Game {
    max_red: usize,
    max_green: usize,
    max_blue: usize,
}

impl Game {
    fn new() -> Self {
        Game {
            max_red: 0,
            max_green: 0,
            max_blue: 0,
        }
    }
}
