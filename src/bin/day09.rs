use std::io::Read;
use std::cmp::{min, max};


fn parse(filename: &str) -> Vec<[i64; 2]> {
    let mut file = std::fs::File::open(filename).expect("failed to open file");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("failed to read file to string");

    s.trim().lines().map(|line| line.split(',').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>().try_into().unwrap()).collect()
}

fn part1(tup: &[[i64; 2]]) -> usize {
    let mut r = vec![];
    for p1 in tup {
        for p2 in tup {
            let dx = max(p1[0], p2[0]) - min(p1[0], p2[0]) + 1;
            let dy = max(p1[1], p2[1]) - min(p1[1], p2[1]) + 1;
            r.push(dx * dy);
        }
    }
    r.into_iter().max().unwrap() as usize
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Rectangle {
    upper_left: [i64; 2],
    lower_right: [i64; 2],
}

impl Rectangle {
    fn new(p1: [i64; 2], p2: [i64; 2]) -> Self {
        Self {
            upper_left: [min(p1[0], p2[0]), min(p1[1], p2[1])],
            lower_right: [max(p1[0], p2[0]), max(p1[1], p2[1])],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Line {
    p1: [i64; 2],
    p2: [i64; 2],
}

impl Line {
    fn new(p1: [i64; 2], p2: [i64; 2]) -> Self {
        if p1[0] + p1[1] < p2[0] + p2[1] {
            Self { p1, p2 }
        } else {
            Self { p1: p2, p2: p1 }
        }
    }

    fn is_horizontal(&self) -> bool {
        self.p1[1] == self.p2[1]
    }

    fn is_vertical(&self) -> bool {
        self.p1[0] == self.p2[0]
    }
}

// A rectangle RE is inside a the union of several other rectangles if the intersection of RE with
// the union is equal to RE.
// How to find union of rectangles that covers the drawn shape? Raster it?

fn part2(tup: &[[i64; 2]]) -> i64 {
    0
}

fn main() {
    let tup = parse("inputs/day09a.txt");
    let answer1 = part1(&tup);
    let answer2 = part2(&tup);
    println!("Examples:");
    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);

    let tup = parse("inputs/day09.txt");
    let answer1 = part1(&tup);
    let answer2 = part2(&tup);
    println!("Challenges:");
    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}
