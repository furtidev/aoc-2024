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

// *very* specialized data structure to handle grids in a clean way (man.. i should've wrote a utility library)
struct Grid {
    item: Vec<Vec<char>>,
}

impl Grid {
    pub fn get2d(&self, row: usize, col: usize, dx: isize, dy: isize) -> Option<&char> {
        let y = row.checked_add_signed(dy);
        let x = col.checked_add_signed(dx);

        if !(y.is_some() && x.is_some()) {
            return None;
        }

        if let Some(first) = self.item.get(y.unwrap()) {
            return first.get(x.unwrap());
        }

        None
    }
}

fn create_ctx(filepath: &str) -> Grid {
    let lines = read_file_to_lines(filepath).unwrap();
    let mut ctx: Grid = Grid { item: vec![] };

    for line in lines.flatten() {
        let char_vec: Vec<char> = line.chars().collect();
        ctx.item.push(char_vec);
    }

    return ctx;
}

fn part1(filepath: &str) -> i32 {
    let mut occur: i32 = 0;

    let ctx: Grid = create_ctx(filepath);

    for (row, level) in ctx.item.iter().enumerate() {
        for (col, &val) in level.iter().enumerate() {
            if val == 'X' {
                // i could hardcode the directions..
                for dy in -1..2 as isize {
                    for dx in -1..2 as isize {
                        if !(dy == 0 && dx == 0) {
                            if ctx.get2d(row, col, dx, dy) == Some(&'M')
                                && ctx.get2d(row, col, dx * 2, dy * 2) == Some(&'A')
                                && ctx.get2d(row, col, dx * 3, dy * 3) == Some(&'S')
                            {
                                occur += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    return occur;
}

fn part2(filepath: &str) -> i32 {
    let mut occur: i32 = 0;

    let ctx: Grid = create_ctx(filepath);

    for (row, level) in ctx.item.iter().enumerate() {
        for (col, &val) in level.iter().enumerate() {
            if val == 'A' {
                let dul = match ctx.get2d(row, col, -1, -1) {
                    Some(n) => *n,
                    None => continue,
                };
                let dur = match ctx.get2d(row, col, 1, -1) {
                    Some(n) => *n,
                    None => continue,
                };
                let ddl = match ctx.get2d(row, col, -1, 1) {
                    Some(n) => *n,
                    None => continue,
                };
                let ddr = match ctx.get2d(row, col, 1, 1) {
                    Some(n) => *n,
                    None => continue,
                };
                let sum = [dul, dur, ddl, ddr];
                let pattern: String = sum.iter().collect();
                if pattern == "MMSS" || pattern == "MSMS" || pattern == "SSMM" || pattern == "SMSM"
                {
                    occur += 1;
                }
            }
        }
    }

    return occur;
}

#[test]
fn test_part1() {
    let result = part1("./example.txt");
    assert_eq!(result, 18);
}

#[test]
fn test_part2() {
    let result = part2("./example.txt");
    assert_eq!(result, 9);
}
