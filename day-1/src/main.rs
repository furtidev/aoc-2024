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

fn create_ctx(filepath: &str) -> Data {
    let lines = read_file_to_lines(filepath).unwrap();
    let mut ctx: Data = Data {
        lhs: vec![],
        rhs: vec![],
    };

    for line in lines.flatten() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        ctx.lhs.push(nums[0]);
        ctx.rhs.push(nums[1]);
    }

    ctx.lhs.sort();
    ctx.rhs.sort();

    return ctx;
}

#[derive(Debug)]
struct Data {
    lhs: Vec<i32>,
    rhs: Vec<i32>,
}

fn part1(filepath: &str) -> i32 {
    let mut ctx = create_ctx(filepath);
    let mut sum: i32 = 0;

    while !ctx.lhs.is_empty() && !ctx.rhs.is_empty() {
        let lhs = *(ctx.lhs.first().unwrap());
        let rhs = *(ctx.rhs.first().unwrap());

        sum += (lhs - rhs).abs();

        ctx.lhs.remove(0);
        ctx.rhs.remove(0);
    }

    return sum;
}

fn part2(filepath: &str) -> i32 {
    let mut ctx = create_ctx(filepath);
    let mut sum: i32 = 0;

    while !ctx.lhs.is_empty() {
        let lhs = *(ctx.lhs.first().unwrap());

        let occurences: i32 = ctx
            .rhs
            .iter()
            .filter(|&n| *n == lhs)
            .count()
            .try_into()
            .unwrap();

        sum += lhs * occurences;

        ctx.lhs.remove(0);
    }

    return sum;
}

// Tests
#[test]
fn test_part1() {
    let result = part1("./example.txt");
    assert_eq!(result, 11);
}

#[test]
fn test_part2() {
    let result = part2("./example.txt");
    assert_eq!(result, 31);
}
