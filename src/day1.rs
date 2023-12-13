use std::{collections::HashMap, fs};

pub fn get_line_calibration(line: &str) -> Option<u8> {
    let mut first_digit: Option<char> = None;
    let mut last_digit: Option<char> = None;
    for c in line.chars() {
        if c.is_digit(10) {
            if first_digit.is_none() {
                first_digit = Some(c);
            }

            last_digit = Some(c);
        }
    }
    match (first_digit, last_digit) {
        (Some(first_digit), Some(last_digit)) => {
            let digit_str = format!("{}{}", first_digit, last_digit);
            digit_str.parse().ok()
        }
        _ => None,
    }
}

pub fn get_doc_calibration(filename: String) -> u32 {
    let mut total_calibration: u32 = 0;

    if let Ok(contents) = fs::read_to_string(filename) {
        for line in contents.lines() {
            if let Some(calibration) = get_line_calibration(line) {
                total_calibration = total_calibration.wrapping_add(calibration as u32);
            }
        }
    } else {
        eprintln!("Failed to read file.")
    }
    total_calibration
}

pub fn get_real_line_calibration(line: &str) -> Option<u8> {
    let word_to_digit = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut first_digit: Option<u32> = None;
    let mut last_digit: Option<u32> = None;

    for (i, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            let digit = c.to_digit(10).unwrap();
            if first_digit.is_none() {
                first_digit = Some(digit);
            }

            last_digit = Some(digit);
        }
        for j in i..line.len() {
            if i < j {
                let word = &line[i..j + 1];
                if word_to_digit.contains_key(word) {
                    let digit = word_to_digit.get(word);
                    if first_digit.is_none() {
                        first_digit = digit.copied();
                    }
                    last_digit = digit.copied();
                }
            }
        }
    }
    match (first_digit, last_digit) {
        (Some(first_digit), Some(last_digit)) => {
            let digit_str = format!("{}{}", first_digit, last_digit);
            digit_str.parse().ok()
        }
        _ => None,
    }
}

pub fn get_real_doc_calibration(filename: String) -> u32 {
    let mut total_calibration: u32 = 0;

    if let Ok(contents) = fs::read_to_string(filename) {
        for line in contents.lines() {
            if let Some(calibration) = get_real_line_calibration(line) {
                total_calibration = total_calibration.wrapping_add(calibration as u32);
            }
        }
    } else {
        eprintln!("Failed to read file.")
    }
    total_calibration
}
