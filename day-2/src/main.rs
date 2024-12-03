use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;
use std::str;

fn main() {
    let p1 = part1("./input.txt");
    let p2 = part2("./input.txt");
    println!("Answer of Part 1 => {}\nAnswer of Part 2 => {}", p1, p2);
}

fn read_file_to_lines<P: AsRef<Path>>(filename: P) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn create_ctx(filepath: &str) -> Vec<Vec<i32>> {
    let lines = read_file_to_lines(filepath).unwrap();
    let mut ctx: Vec<Vec<i32>> = vec![];

    for line in lines.flatten() {
        let nums: Vec<i32> = line.split(" ").map(|n| n.parse::<i32>().unwrap()).collect();

        ctx.push(nums);
    }

    return ctx;
}

fn is_within_bounds(x: i32, y: i32) -> bool {
    if (x - y).abs() > 3 || (x - y).abs() < 1 || (x - y) == 0 {
        return false;
    }
    true
}

fn is_safe(level: Vec<i32>) -> bool {
    for (idx, &val) in level.iter().enumerate() {
        let previous = if idx > 0 {
            level.get(idx - 1)
        } else {
            Option::None
        };
        let next = level.get(idx + 1);

        // conditions for checking if it's safe
        if previous.is_none() {
            let n = *(next.unwrap());
            if !is_within_bounds(n, val) {
                return false;
            }
        } else if next.is_none() {
            let n = *(previous.unwrap());
            if !is_within_bounds(n, val) {
                return false;
            }
        } else {
            // unwrap and deference now because we don't care about Option data anymore.
            let previous = *(previous.unwrap());
            let next = *(next.unwrap());

            if previous < val && !(next > val) {
                return false;
            } else if previous > val && !(next < val) {
                return false;
            } else if !is_within_bounds(previous, val) || !is_within_bounds(next, val) {
                return false;
            }
        }
    }

    true
}

fn is_safe_tolerant(level: Vec<i32>) -> bool {
    let mut violations: Vec<usize> = vec![];
    for (idx, &val) in level.iter().enumerate() {
        let previous = if idx > 0 {
            level.get(idx - 1)
        } else {
            Option::None
        };
        let next = level.get(idx + 1);

        if previous.is_none() {
            // next one
            let n = *(next.unwrap());
            // the next one after
            let x = *(level.get(idx + 2).unwrap());
            if !is_within_bounds(n, val) {
                // println!("bound check failed");
                violations.push(idx);
            }

            // These checks weren't needed in part one, but they are needed now :p edge cases, am i right?
            if val > x && n < x {
                // check if it's decreasing, and whether that's maintained
                violations.push(idx);
            } else if val < x && n > x {
                // vice versa
                violations.push(idx);
            }
        } else if next.is_none() {
            // previous one
            let n = *(previous.unwrap());
            // // the one before the previous one
            let x = *(level.get(idx - 2).unwrap());
            if !is_within_bounds(n, val) {
                violations.push(idx);
            }

            // you know the deal..
            if x > val && n > x {
                // check if it's decreasing, and whether that's maintained
                violations.push(idx);
            } else if x < val && n < x {
                // vice versa
                violations.push(idx);
            }
        } else {
            // unwrap and deference now because we don't care about Option data anymore.
            let previous = *(previous.unwrap());
            let next = *(next.unwrap());

            if previous < val && !(next > val) {
                violations.push(idx);
            } else if previous > val && !(next < val) {
                violations.push(idx);
            } else if !is_within_bounds(previous, val) || !is_within_bounds(next, val) {
                violations.push(idx);
            }
        }
    }

    // try to solve the violations if possible.
    let mut safe: bool = false;
    for &val in violations.iter() {
        let mut modified = level.clone();
        modified.remove(val);
        if is_safe(modified.clone()) {
            safe = true;
            break;
        }
    }
    if violations.len() > 0 {
        return safe;
    }
    true
}

fn part1(filepath: &str) -> i32 {
    let mut sum: i32 = 0;
    let ctx: Vec<Vec<i32>> = create_ctx(filepath);

    for level in ctx {
        let safe = is_safe(level);
        // mark as safe
        sum += if safe { 1 } else { 0 };
    }

    return sum;
}

fn part2(filepath: &str) -> i32 {
    let mut sum: i32 = 0;
    let ctx: Vec<Vec<i32>> = create_ctx(filepath);

    for level in ctx {
        let safe = is_safe_tolerant(level);
        // mark as safe
        sum += if safe { 1 } else { 0 };
    }

    return sum;
}

#[test]
fn test_part1() {
    let result = part1("./example.txt");
    assert_eq!(result, 2);
}

#[test]
fn test_part2() {
    let result = part2("./example.txt");
    assert_eq!(result, 4);
}
