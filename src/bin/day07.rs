use std::io::Read;
use std::collections::{HashSet, HashMap};


fn parse(filename: &str) -> Vec<Vec<char>> {
    let mut file = std::fs::File::open(filename).expect("failed to open file");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("failed to read file to string");
    s.trim().lines().map(|line| line.chars().collect()).collect()
}

fn part1(rows: &[Vec<char>]) -> i64 {
    // Start at the 'S'
    let (start_idx, _) = rows[0].iter().enumerate().find(|&(_i, c)| *c == 'S').unwrap();
    let mut water_idxs = vec![start_idx];
    let mut cntr = 0;
    for row in rows[1..].iter() {
        let mut new_water_idxs = HashSet::new();
        for wi in &water_idxs {
            if row[*wi] == '^' {
                new_water_idxs.insert(wi-1);
                new_water_idxs.insert(wi+1);
                cntr += 1;
            } else {
                new_water_idxs.insert(*wi);
            }
        }
        water_idxs = new_water_idxs.into_iter().collect();
    }
    cntr
}

fn part2(rows: &[Vec<char>]) -> usize {
    // Start at the 'S'
    let (start_idx, _) = rows[0].iter().enumerate().find(|&(_i, c)| *c == 'S').unwrap();
    let mut paths = HashMap::new();
    // hashmap key is idx, value is count of paths that lead here.
    paths.insert(start_idx, 1);
    for row in rows[1..].iter() {
        let mut new_paths = HashMap::new();
        for i in paths.keys() {
            let count = paths[i];
            if row[*i] == '^' {
                if let Some(v) = new_paths.get_mut(&(i - 1)) {
                    *v += count;
                } else {
                    new_paths.insert(i - 1, count);
                }
                if let Some(v) = new_paths.get_mut(&(i + 1)) {
                    *v += count;
                } else {
                    new_paths.insert(i + 1, count);
                }
            } else {
                if let Some(v) = new_paths.get_mut(&i) {
                    *v += count;
                } else {
                    new_paths.insert(*i, count);
                }
            }
        }
        paths = new_paths;
    }
    paths.values().sum::<usize>()
}

fn main() {
    let map = parse("inputs/day07a.txt");
    let answer1 = part1(&map);
    let answer2 = part2(&map);
    println!("Examples:");
    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);

    let map = parse("inputs/day07.txt");
    let answer1 = part1(&map);
    let answer2 = part2(&map);
    println!("Challenges:");
    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}
