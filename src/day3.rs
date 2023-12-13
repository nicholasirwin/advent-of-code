use std::{
    collections::{HashMap, HashSet},
    fs, vec,
};
// build 2D array

// for each row,
// for each "part",
// search all adjacent spots,

struct Direction {
    x: i32,
    y: i32,
}

pub fn p1(filename: &str) -> i32 {
    let mut res = 0;

    if let Ok(contents) = fs::read_to_string(filename) {
        let y_size = contents.lines().count();
        let x_size = contents.lines().collect::<Vec<_>>()[0].len();

        let mut grid = vec![vec!['_'; x_size]; y_size];

        for (y_inv, line) in contents.lines().enumerate() {
            let y = y_size - y_inv - 1;
            for (x, c) in line.chars().enumerate() {
                grid[y][x] = c;
            }
        }

        let dirs = vec![
            Direction { x: 0, y: 1 },
            Direction { x: 0, y: -1 },
            Direction { x: 1, y: 1 },
            Direction { x: 1, y: 0 },
            Direction { x: 1, y: -1 },
            Direction { x: -1, y: -1 },
            Direction { x: -1, y: 0 },
            Direction { x: -1, y: 1 },
        ];

        for (y_inv, line) in contents.lines().enumerate() {
            let y: usize = y_size - y_inv - 1;

            let mut curr_num = String::from("");
            let mut is_part = false;

            for (x, c) in line.chars().enumerate() {
                if (c == '.') || (!c.is_numeric()) {
                    if (is_part) & (curr_num != "") {
                        res += curr_num.parse::<i32>().unwrap();
                    }
                    curr_num = "".to_string();
                    is_part = false;
                } else {
                    curr_num += &c.to_string();
                    for dir in &dirs {
                        let new_loc = (x as i32 + dir.x, y as i32 + dir.y);
                        if (new_loc.0 >= 0)
                            & (new_loc.0 < x_size as i32)
                            & (new_loc.1 >= 0)
                            & (new_loc.1 < y_size as i32)
                        {
                            let char_at_new_loc = grid[new_loc.1 as usize][new_loc.0 as usize];
                            if (char_at_new_loc != '.') & (!char_at_new_loc.is_numeric()) {
                                is_part = true;
                            }
                        }
                    }
                }
                if x == x_size - 1 {
                    if (is_part) & (curr_num != "") {
                        res += curr_num.parse::<i32>().unwrap();
                    }
                }
            }
        }
    }
    res
}

pub fn p2(filename: &str) -> i32 {
    let mut res = 0;

    if let Ok(contents) = fs::read_to_string(filename) {
        let y_size = contents.lines().count();
        let x_size = contents.lines().collect::<Vec<_>>()[0].len();

        let mut grid = vec![vec!['_'; x_size]; y_size];

        // map every gear loc to vec of adj parts,
        let mut gears = vec![];

        for (y_inv, line) in contents.lines().enumerate() {
            let y = y_size - y_inv - 1;
            for (x, c) in line.chars().enumerate() {
                grid[y][x] = c;
                if c == '*' {
                    gears.push((x as i32, y as i32));
                }
            }
        }

        let mut gears_to_adj_parts: HashMap<(i32, i32), HashSet<i32>> = HashMap::new();

        for gear in gears {
            gears_to_adj_parts.insert(gear, HashSet::new());
        }

        let dirs = vec![
            Direction { x: 0, y: 1 },
            Direction { x: 0, y: -1 },
            Direction { x: 1, y: 1 },
            Direction { x: 1, y: 0 },
            Direction { x: 1, y: -1 },
            Direction { x: -1, y: -1 },
            Direction { x: -1, y: 0 },
            Direction { x: -1, y: 1 },
        ];

        for (y_inv, line) in contents.lines().enumerate() {
            let y: usize = y_size - y_inv - 1;

            let mut curr_num = String::from("");

            let mut adj_gears = vec![];

            for (x, c) in line.chars().enumerate() {
                if !c.is_numeric() {
                    if curr_num != "" {
                        for &_gear in &adj_gears {
                            gears_to_adj_parts
                                .get_mut(&_gear)
                                .unwrap()
                                .insert(curr_num.parse::<i32>().unwrap());
                        }
                    }
                    curr_num.clear();
                    adj_gears.clear();
                } else {
                    curr_num += &c.to_string();
                    for dir in &dirs {
                        let new_loc = (x as i32 + dir.x, y as i32 + dir.y);
                        if (new_loc.0 >= 0)
                            & (new_loc.0 < x_size as i32)
                            & (new_loc.1 >= 0)
                            & (new_loc.1 < y_size as i32)
                        {
                            let char_at_new_loc = grid[new_loc.1 as usize][new_loc.0 as usize];
                            if char_at_new_loc == '*' {
                                // adj_gears.push(&new_loc);
                                adj_gears.push((new_loc.0, new_loc.1));
                            }
                        }
                    }
                }
                if x == x_size - 1 {
                    if curr_num != "" {
                        for &_gear in &adj_gears {
                            gears_to_adj_parts
                                .get_mut(&_gear)
                                .unwrap()
                                .insert(curr_num.parse::<i32>().unwrap());
                        }
                    }
                }
            }
        }

        for (_, adj_parts) in gears_to_adj_parts {
            if adj_parts.len() == 2 {
                let mut product = 1;
                for part in adj_parts {
                    product *= part;
                }
                res += product;
            }
        }
    }
    res
}
