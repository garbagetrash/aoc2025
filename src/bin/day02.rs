use std::collections::HashSet;
use std::io::Read;
use std::time::Instant;


fn parse(filename: &str) -> Vec<[usize; 2]> {
    let mut file = std::fs::File::open(filename).expect("failed to open file");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("failed to read file to string");
    s.trim().split(',').map(|range| {
        range.split('-').map(|n| {
            n.parse::<usize>().expect("failed to parse usize")
        }).collect::<Vec<usize>>().try_into().unwrap()
    }).collect()
}

fn part1(id_ranges: &[[usize; 2]]) -> usize {
    let mut output = 0;
    for range in id_ranges {
        let mut i = range[0];
        let end = range[1];
        while i <= end {
            let ndigits = i.ilog10() as usize + 1;
            if ndigits % 2 == 0 {
                // Even number of digits
                let n2 = ndigits/2;
                let k = 10_usize.pow(n2 as u32);
                let left = i / k;
                let right = i % k;
                if left == right {
                    output += i;
                    i = (left + 1) * k + left + 1;
                } else if left > right {
                    i = left * k + left;
                } else {
                    i = (left + 1) * k + left + 1;
                }
            } else {
                // Skip intervals that don't even have even number of digits
                i = 10_usize.pow(ndigits as u32);
            }
        }
    }
    output
}

fn check_valid(_id: usize, length: usize) -> bool {
    let id = _id.to_string();
    let nrepeats = id.len() / length;
    if nrepeats < 2 {
        return true;
    }
    let pattern = &id[..length];
    let mut valid = false;
    for idx in 1..nrepeats {
        if pattern != &id[length*idx..length*(idx+1)] {
            valid = true;
            break
        }
    }
    valid
}

fn part2(id_ranges: &[[usize; 2]]) -> usize {
    let mut invalids = HashSet::new();
    for range in id_ranges {
        let end = range[1];
        let n_end_digits = end.ilog10() as usize + 1;
        let maxlength = n_end_digits / 2 + 1;
        for length in 1..maxlength {
            let mut i = range[0];
            while i <= end {
                let ndigits = i.ilog10() as usize + 1;
                if ndigits % length != 0 {
                    // Skip interval to next valid length
                    i = 10_usize.pow(ndigits as u32);
                }
                let k = 10_usize.pow(length as u32);
                let kk = 10_usize.pow((ndigits - length) as u32);
                let left = i / kk;
                let right = i % k;
                if right < left {
                    //println!("right < left ||| i: {}, right: {}, left: {}, length: {}, k: {}, kk: {}", i, right, left, length, k, kk);
                    i = (i / k) * k + left - 1;
                } else if right > left {
                    //println!("right > left ||| i: {}, right: {}, left: {}, length: {}, k: {}, kk: {}", i, right, left, length, k, kk);
                    //i += kk - 1;
                    //i = (i / k) * k;
                } else {
                    // left == right => might be invalid
                    if !check_valid(i, length) {
                        invalids.insert(i);
                    }
                }
                i += 1;
            }
        }
    }
    invalids.into_iter().sum::<usize>()
}

fn main() {
    let t0 = Instant::now();

    let id_ranges = parse("inputs/day02a.txt");
    let answer1 = part1(&id_ranges);
    let answer2 = part2(&id_ranges);
    println!("Examples:");
    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);

    let id_ranges = parse("inputs/day02.txt");
    let answer1 = part1(&id_ranges);
    let answer2 = part2(&id_ranges);
    println!("Challenges:");
    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);

    println!("Time: {} ms", 1000.0 * t0.elapsed().as_secs_f64());
}
