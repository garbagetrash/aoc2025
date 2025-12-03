use std::io::Read;

fn parse(filename: &str) -> Vec<Vec<usize>> {
    let mut file = std::fs::File::open(filename).expect("failed to open file");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("failed to read file to string");
    s.trim().split(',').map(|range| {
        range.split('-').map(|n| {
            n.parse::<usize>().expect("failed to parse usize")
        }).collect()
    }).collect()
}

fn part1(id_ranges: &[Vec<usize>]) -> usize {
    let mut output = 0;
    for range in id_ranges {
        for i in range[0]..range[1]+1 {
            let s = i.to_string();
            let n = s.len();
            let n2 = n/2;
            if s[..n2] == s[n2..] {
                output += i;
            }
        }
    }
    output
}

fn str_chunks(s: &str, n: usize) -> Vec<&str> {
    let mut idx = 0;
    let mut output = vec![];
    while idx < s.len() {
        output.push(&s[idx..idx+n]);
        idx += n;
    }
    output
}

fn part2(id_ranges: &[Vec<usize>]) -> usize {
    let mut output = 0;
    for range in id_ranges {
        for i in range[0]..range[1]+1 {
            let s = i.to_string();
            let n = s.len();
            let mut invalid = false;
            for nn in 2..n+1 {
                if n%nn == 0 {
                    let nletters = n/nn;
                    let siter = str_chunks(s.as_str(), nletters);
                    let seq = siter[0];
                    invalid = siter[1..].iter().fold(true, |acc, seq2| acc & (seq == *seq2));
                    if invalid {
                        break
                    }
                }
            }
            if invalid {
                output += i;
            }
        }
    }
    output
}

fn main() {
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
}
