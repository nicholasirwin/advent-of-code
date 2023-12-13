use std::cmp::max;
use std::fs;

const RED_CUBES: u32 = 12;
const GREEN_CUBES: u32 = 13;
const BLUE_CUBES: u32 = 14;

pub fn is_valid_draw(red_draw: u32, green_draw: u32, blue_draw: u32) -> bool {
    (red_draw <= RED_CUBES) & (green_draw <= GREEN_CUBES) & (blue_draw <= BLUE_CUBES)
}

pub fn all_valid_draws(draws: &str) -> bool {
    for handfull in draws.split(';').into_iter() {
        let mut red_draw: u32 = 0;
        let mut green_draw: u32 = 0;
        let mut blue_draw: u32 = 0;

        let words = handfull.split(' ').collect::<Vec<&str>>();
        let words = &words[1..];

        let num_spaces = words.len();

        for splitter in 1..num_spaces {
            if splitter % 2 == 1 {
                let num_cubes = words[splitter - 1].parse().unwrap();
                let mut color = words[splitter];

                if color.chars().last() == Some(',') {
                    color = &color[..color.len() - 1];
                }

                match color {
                    "red" => red_draw = num_cubes,
                    "green" => green_draw = num_cubes,
                    "blue" => blue_draw = num_cubes,
                    _ => (),
                }
            }
        }
        let is_valid_game = is_valid_draw(red_draw, green_draw, blue_draw);
        if !is_valid_game {
            return false;
        }
    }
    true
}

pub fn power(draws: &str) -> u32 {
    let mut min_red: u32 = 0;
    let mut min_green: u32 = 0;
    let mut min_blue: u32 = 0;

    for handfull in draws.split(';').into_iter() {
        let words = handfull.split(' ').collect::<Vec<&str>>();
        let words = &words[1..];

        let num_spaces = words.len();

        for splitter in 1..num_spaces {
            if splitter % 2 == 1 {
                let num_cubes = words[splitter - 1].parse().unwrap();
                let mut color = words[splitter];

                if color.chars().last() == Some(',') {
                    color = &color[..color.len() - 1];
                }

                match color {
                    "red" => min_red = max(min_red, num_cubes),
                    "green" => min_green = max(min_green, num_cubes),
                    "blue" => min_blue = max(min_blue, num_cubes),
                    _ => (),
                }
            }
        }
    }
    min_red * min_blue * min_green
}

pub fn process_line(line: &str) -> (u32, &str) {
    let pieces = line.split(':').collect::<Vec<&str>>();
    let draws = pieces[1];

    let game_num = pieces[0].split(' ').last().unwrap().parse().unwrap();

    (game_num, draws)
}

pub fn process_doc(filename: &str) -> u32 {
    let mut res = 0;

    if let Ok(contents) = fs::read_to_string(filename) {
        for line in contents.lines() {
            let (game_num, draws) = process_line(line);
            if all_valid_draws(draws) {
                res += game_num;
            }
        }
    }
    res
}

pub fn process_doc_power(filename: &str) -> u32 {
    let mut res = 0;

    if let Ok(contents) = fs::read_to_string(filename) {
        for line in contents.lines() {
            let (game_num, draws) = process_line(line);
            let power = power(draws);
            res += power;
        }
    }
    res
}
