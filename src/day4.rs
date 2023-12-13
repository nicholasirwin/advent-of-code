use std::collections::HashMap;
use std::{collections::HashSet, fs};

pub fn parse_line(line: &str) -> (HashSet<i32>, HashSet<i32>) {
    let mut my_numbers = HashSet::new();
    let mut winning_numbers = HashSet::new();

    let numbers = line.split(':').collect::<Vec<&str>>()[1];

    let left_and_right = numbers.split('|').collect::<Vec<&str>>();

    let my_numbers_ = left_and_right[0];
    for number in my_numbers_.split(' ').into_iter() {
        if number != "" {
            my_numbers.insert(number.parse().unwrap());
        }
    }

    let winning_numbers_ = left_and_right[1];
    for number in winning_numbers_.split(' ').into_iter() {
        if number != "" {
            winning_numbers.insert(number.parse().unwrap());
        }
    }

    (my_numbers, winning_numbers)
}

pub fn get_card_score_p1(my_numbers: HashSet<i32>, winning_numbers: HashSet<i32>) -> i32 {
    let count = my_numbers
        .iter()
        .filter(|number| winning_numbers.contains(number))
        .count() as i32;

    if count == 0 {
        return count;
    }

    2_i32.pow(count as u32 - 1)
}

pub fn solve_p1(filename: &str) -> i32 {
    let mut res = 0;
    if let Ok(contents) = fs::read_to_string(filename) {
        for line in contents.lines() {
            let (my_numbers, winning_numbers) = parse_line(line);
            let score = get_card_score_p1(my_numbers, winning_numbers);
            res += score;
        }
    }
    res
}

pub fn get_card_score_p2(my_numbers: HashSet<i32>, winning_numbers: HashSet<i32>) -> i32 {
    my_numbers
        .iter()
        .filter(|number| winning_numbers.contains(number))
        .count() as i32
}

pub fn solve_p2(filename: &str) -> i32 {
    let mut res = 0;

    if let Ok(contents) = fs::read_to_string(filename) {
        let original_cards = contents.lines().collect::<Vec<_>>();
        let mut copies: Vec<usize> = (0..original_cards.len()).collect();

        let card_scores: HashMap<usize, i32> = original_cards
            .iter()
            .enumerate()
            .map(|(idx, line)| {
                let (my_numbers, winning_numbers) = parse_line(line);
                (idx, get_card_score_p2(my_numbers, winning_numbers))
            })
            .collect::<HashMap<_, _>>();

        while copies.len() > 0 {
            res += 1;

            let card_idx_to_process = copies.pop().unwrap();
            let card_score = card_scores[&card_idx_to_process] as usize;

            let copies_: Vec<usize> = (card_idx_to_process + 1..original_cards.len()).collect();
            copies.extend_from_slice(&copies_[0..card_score]);
        }
    }
    res
}
