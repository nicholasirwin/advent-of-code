use std::{cmp::min, collections::HashSet, fs};

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

pub fn get_card_score(my_numbers: HashSet<i32>, winning_numbers: HashSet<i32>) -> i32 {
    let mut count = 0;

    for number in &my_numbers {
        if winning_numbers.contains(number) {
            count += 1;
        }
    }

    count
}

pub fn get_copies(curr_score: i32, lines: Vec<&str>) -> i32 {
    let first_line = lines[0];
    let (my_numbers, winning_numbers) = parse_line(first_line);
    let score = get_card_score(my_numbers, winning_numbers);

    if score == 0 {
        return 1;
    }
    // println!("{}", score);

    let last_line_idx = min(score + 1, lines.len() as i32);

    let copies = lines[1..last_line_idx as usize].to_vec();

    // println!("{:?}", copies);

    let mut sum = 0;
    for idx in 0..copies.len() {
        sum += get_copies(curr_score, copies[idx..].to_vec());
    }
    sum

    // get_copies(curr_score + copies.len() as i32, copies)
}

pub fn run(filename: &str) -> i32 {
    let mut res = 0;
    if let Ok(contents) = fs::read_to_string(filename) {
        let lines = contents.lines().collect::<Vec<_>>();

        for idx in 0..lines.len() {
            // println!("Line idx: {}", idx);
            let total_cards = get_copies(1, lines[idx..].to_vec());
            // println!("{}", total_cards);
            res += total_cards;
        }
    }
    res
}
