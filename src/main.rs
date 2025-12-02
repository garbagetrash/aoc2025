use std::io::Read;
use std::cmp::{min, max};


fn parse(filename: &str) -> Vec<i64> {
    let mut file = std::fs::File::open(filename).expect("failed to open file");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("failed to read to string");
    s.lines().map(|line| {
        let mut out = line[1..].parse::<i64>().expect("failed to parse i64");
        if line.chars().next().unwrap() == 'L' {
            out *= -1;
        }
        out
    }).collect()
}

fn spin(_start: i64, new: i64) -> i64 {
    let tmp = _start + new;
    let mut output = max(_start, tmp) / 100 - min(_start, tmp) / 100;
    if new < 0 {
        if tmp % 100 == 0 {
            output += 1;
        }
        if _start % 100 == 0 {
            output -= 1;
        }
    }
    output
}

fn part1(sequence: &[i64]) -> i64 {
    let mut last = 10000000000000050;
    let mut cntr = 0;
    for n in sequence {
        last += n;
        if last % 100 == 0 {
            cntr += 1;
        }
    }
    cntr
}

fn part2(sequence: &[i64]) -> i64 {
    let mut last = 100000000000050;
    let mut output = 0;
    for n in sequence {
        output += spin(last, *n);
        last += *n;
    }
    output
}

fn main() {
    // Examples
    let sequence = parse("inputs/day01a.txt");
    println!("Part 1: {}", part1(&sequence));
    println!("Part 2: {}", part2(&sequence));

    // Real input
    let sequence = parse("inputs/day01.txt");
    println!("Part 1: {}", part1(&sequence));
    println!("Part 2: {}", part2(&sequence));
}
