use std::io::Read;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Ops {
    Add,
    Mul,
}

fn parse(filename: &str) -> Vec<(Ops, Vec<i64>)> {
    let mut file = std::fs::File::open(filename).expect("failed to open file");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("failed to read file to string");
    let mut tmp = vec![];
    for line in s.trim().lines() {
        tmp.push(line.split_whitespace().collect::<Vec<_>>());
    }

    let mut nums = vec![];
    let mut ops = vec![];
    for (i, t) in tmp.iter().enumerate() {
        if i == tmp.len() - 1 {
            // Handle operators
            ops = t.into_iter().map(|&o| {
                match o {
                    "+" => Ops::Add,
                    "*" => Ops::Mul,
                    _ => panic!("invalid op"),
                }
            }).collect();
        } else {
            nums.push(t.into_iter().map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>());
        }
    }

    let mut output = vec![];
    let mut vnums = vec![vec![0; nums.len()]; nums[0].len()];
    for i in 0..nums[0].len() {
        for j in 0..nums.len() {
            vnums[i][j] = nums[j][i];
        }
    }
    for (i, col) in vnums.iter().enumerate() {
        output.push((ops[i], col.clone()));
    }
    output
}

fn part1(rows: &[(Ops, Vec<i64>)]) -> i64 {
    let mut output = 0;
    for col in rows {
        output += match col.0 {
            Ops::Add => col.1.iter().sum::<i64>(),
            Ops::Mul => col.1.iter().product::<i64>(),
        }
    }
    output
}

fn parse2(filename: &str) -> Vec<(Ops, Vec<i64>)> {
    let mut file = std::fs::File::open(filename).expect("failed to open file");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("failed to read file to string");

    let tmp: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();

    let ncols = tmp[0].len();
    let nrows = tmp.len() - 1;
    let mut vnums: Vec<Vec<i64>> = vec![];
    let mut problem_nums = vec![];
    for c in 0..ncols {
        let mut num = vec![];
        for r in 0..nrows {
            num.push(tmp[r][c]);
        }
        let num_str = num.into_iter().collect::<String>().trim().to_string();
        if num_str.len() > 0 {
            problem_nums.push(num_str.parse::<i64>().unwrap());
        } else {
            // Done with problem
            vnums.push(std::mem::replace(&mut problem_nums, vec![]));
        }
    }
    if problem_nums.len() > 0 {
        vnums.push(std::mem::replace(&mut problem_nums, vec![]));
    }

    // Indexes of operators indicate last number in a problem
    let mut ops = vec![];
    for c in &tmp[nrows] {
        match *c {
            '+' => ops.push(Ops::Add),
            '*' => ops.push(Ops::Mul),
            _ => (),
        };
    }

    let mut output = vec![];
    for i in 0..ops.len() {
        output.push((ops[i], vnums[i].clone()));
    }
    output
}

fn part2(rows: &[(Ops, Vec<i64>)]) -> i64 {
    let mut output = 0;
    for col in rows {
        output += match col.0 {
            Ops::Add => col.1.iter().sum::<i64>(),
            Ops::Mul => col.1.iter().product::<i64>(),
        }
    }
    output
}

fn main() {
    let map = parse("inputs/day06a.txt");
    let map2 = parse2("inputs/day06a.txt");
    let answer1 = part1(&map);
    let answer2 = part2(&map2);
    println!("Examples:");
    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);

    let map = parse("inputs/day06.txt");
    let map2 = parse2("inputs/day06.txt");
    let answer1 = part1(&map);
    let answer2 = part2(&map2);
    println!("Challenges:");
    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}
