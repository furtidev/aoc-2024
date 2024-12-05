use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
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

fn part1(filename: &str) -> i32 {
    let mut sum: i32 = 0;
    let mut parse_mode = 0;
    // 0 = rules, 1 = pages

    let lines = read_file_to_lines(filename).unwrap();
    let mut rules = HashMap::<u8, Vec<u8>>::new();
    let mut pages: Vec<Vec<u8>> = vec![];

    for line in lines.flatten() {
        if line.is_empty() {
            parse_mode = 1;
            continue;
        }

        if parse_mode == 0 {
            let nums: Vec<u8> = line.split("|").map(|n| n.parse::<u8>().unwrap()).collect();
            rules.entry(nums[0]).or_default().push(nums[1]);
        } else {
            let nums: Vec<u8> = line.split(",").map(|n| n.parse::<u8>().unwrap()).collect();
            pages.push(nums);
        }
    }

    let valid_updates: Vec<_> = pages
        .iter()
        .filter(|page| {
            let mut valid = true;
            let mut before = HashSet::new();

            for num in page.iter() {
                if let Some(come_after) = rules.get(num) {
                    for rule in come_after {
                        if before.contains(rule) {
                            valid = false;
                        }
                    }
                }
                before.insert(*num);
            }
            return valid;
        })
        .collect();

    for item in valid_updates {
        sum += item[item.len() / 2] as i32;
    }

    return sum;
}

fn part2(filename: &str) -> i32 {
    let mut sum: i32 = 0;
    let mut parse_mode = 0;
    // 0 = rules, 1 = pages

    let lines = read_file_to_lines(filename).unwrap();
    let mut rules = HashMap::<u8, Vec<u8>>::new();
    let mut pages: Vec<Vec<u8>> = vec![];

    for line in lines.flatten() {
        if line.is_empty() {
            parse_mode = 1;
            continue;
        }

        if parse_mode == 0 {
            let nums: Vec<u8> = line.split("|").map(|n| n.parse::<u8>().unwrap()).collect();
            rules.entry(nums[0]).or_default().push(nums[1]);
        } else {
            let nums: Vec<u8> = line.split(",").map(|n| n.parse::<u8>().unwrap()).collect();
            pages.push(nums);
        }
    }

    let invalid_updates: Vec<_> = pages
        .iter()
        .filter(|page| {
            let mut invalid = false;
            let mut before = HashSet::new();

            for num in page.iter() {
                if let Some(come_after) = rules.get(num) {
                    for rule in come_after {
                        if before.contains(rule) {
                            invalid = true;
                        }
                    }
                }
                before.insert(*num);
            }
            return invalid;
        })
        .collect();

    for &item in invalid_updates.iter() {
        let mut sorted = item.clone();
        sorted.sort_by(|a, b| {
            if let Some(rule) = rules.get(a) {
                if rule.contains(b) {
                    return Ordering::Less;
                } else {
                    return Ordering::Greater;
                }
            }
            Ordering::Equal
        });

        sum += sorted[sorted.len() / 2] as i32;
    }

    return sum;
}

#[test]
fn test_part1() {
    let result = part1("./example.txt");
    assert_eq!(result, 143);
}

#[test]
fn test_part2() {
    let result = part2("./example.txt");
    assert_eq!(result, 123);
}
