use std::io::Read;

fn parse(filename: &str) -> Vec<Vec<u64>> {
    let mut file = std::fs::File::open(filename).expect("failed to open file");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("failed to read file to string");
    s.trim().lines().map(|line| {
        line.chars().map(|c| c.to_digit(10).expect("failed to parse u64") as u64).collect()
    }).collect()
}

fn part1(rows: &[Vec<u64>]) -> u64 {
    let mut output = 0;
    for row in rows {
        // Find first digit
        let mut max_digit = row[0];
        let mut max_idx = 0;
        for (i, n) in row[..row.len()-1].iter().enumerate() {
            if *n > max_digit {
                max_digit = *n;
                max_idx = i;
            }
        }

        // Find second digit
        let mut max_digit2 = 0;
        for n in row[max_idx + 1..].iter() {
            if *n > max_digit2 {
                max_digit2 = *n;
            }
        }

        // Joltage
        let joltage = 10 * max_digit + max_digit2;
        output += joltage;
    }
    output
}

fn first_max_digit(digits: &[u64], depth: usize) -> (u64, &[u64]) {
    let mut max_digit = digits[0];
    let mut max_idx = 0;
    for (i, n) in digits[..digits.len()-(depth-1)].iter().enumerate() {
        if *n > max_digit {
            max_digit = *n;
            max_idx = i;
        }
    }
    (max_digit, &digits[max_idx+1..])
}

fn part2(rows: &[Vec<u64>]) -> u64 {
    let mut output = 0;
    for row in rows {
        let mut digits = [0; 12];
        let mut last_slice: &[u64] = &row;
        for i in 0..12 {
            let depth = 12 - i;
            let (new_digit, new_slice) = first_max_digit(last_slice, depth);
            digits[i] = new_digit;
            last_slice = new_slice;
        }

        // Joltage
        let mut joltage = 0;
        for i in 0..12 {
            joltage += 10_u64.pow(11-i as u32) * digits[i];
        }
        output += joltage;
    }
    output
}

fn main() {
    let id_ranges = parse("inputs/day03a.txt");
    let answer1 = part1(&id_ranges);
    let answer2 = part2(&id_ranges);
    println!("Examples:");
    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);

    let id_ranges = parse("inputs/day03.txt");
    let answer1 = part1(&id_ranges);
    let answer2 = part2(&id_ranges);
    println!("Challenges:");
    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}
