use std::io::Read;

type input = (Vec<(usize, usize)>, Vec<usize>);

fn parse(filename: &str) -> input {
    let mut file = std::fs::File::open(filename).expect("failed to open file");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("failed to read file to string");

    let mut part1 = true;
    let mut fresh_ranges = vec![];
    let mut available = vec![];
    for line in s.trim().lines() {
        //println!("line: `{}`", line);
        if line.len() == 0 {
            // Part 2 now
            part1 = false;
            continue;
        }
        if part1 {
            let mut tmp = line.split('-');
            fresh_ranges.push((tmp.next().unwrap().parse::<usize>().unwrap(), tmp.next().unwrap().parse::<usize>().unwrap()));
        } else {
            available.push(line.parse::<usize>().unwrap());
        }
    }
    (fresh_ranges, available)
}

fn part1(tup: &input) -> usize {
    let (fresh_ranges, available) = tup;
    let mut cntr = 0;
    // Just be naive for now
    for a in available {
        for fr in fresh_ranges {
            if fr.0 <= *a && fr.1 >= *a {
                cntr += 1;
                break;
            }
        }
    }
    cntr
}

use aoc2025::{Range, union};

fn part2(tup: &input) -> usize {
    let (fresh_ranges, _available) = tup;
    let fresh_ranges: Vec<_> = fresh_ranges.into_iter().map(|fr| Range::new(fr.0, fr.1)).collect();
    let theunion = union(&fresh_ranges);
    theunion.iter().map(|u| u.len()).sum::<usize>()
}

fn main() {
    let tup = parse("inputs/day05a.txt");
    let answer1 = part1(&tup);
    let answer2 = part2(&tup);
    println!("Examples:");
    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);

    let tup = parse("inputs/day05.txt");
    let answer1 = part1(&tup);
    let answer2 = part2(&tup);
    println!("Challenges:");
    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}
