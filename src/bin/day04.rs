use std::io::Read;

fn parse(filename: &str) -> Vec<Vec<u8>> {
    let mut file = std::fs::File::open(filename).expect("failed to open file");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("failed to read file to string");
    s.trim().lines().map(|line| {
        line.chars().map(|c| {
            if c == '.' {
                0
            } else {
                1
            }
        }).collect()
    }).collect()
}

fn get_neighbors(pos: (i32, i32), map: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let mut dirs = vec![];
    if pos.0 > 0 {
        dirs.push((-1, 0));
        if pos.1 > 0 {
            dirs.push((-1, -1));
        }
        if (pos.1 as usize) < map.len() - 1 {
            dirs.push((-1, 1));
        }
    }
    if (pos.0 as usize) < map[0].len() - 1 {
        dirs.push((1, 0));
        if pos.1 > 0 {
            dirs.push((1, -1));
        }
        if (pos.1 as usize) < map.len() - 1 {
            dirs.push((1, 1));
        }
    }
    if pos.1 > 0 {
        dirs.push((0, -1));
    }
    if (pos.1 as usize) < map.len() - 1 {
        dirs.push((0, 1));
    }

    let mut neighbors = vec![];
    for dir in dirs {
        neighbors.push(((pos.0 + dir.0) as usize, (pos.1 + dir.1) as usize));
    }
    neighbors
}

fn part1(rows: &[Vec<u8>]) -> u64 {
    let mut output = 0;
    for (y, row) in rows.iter().enumerate() {
        for (x, p) in row.iter().enumerate() {
            if *p == 1 {
                let neighbors = get_neighbors((x as i32, y as i32), rows);
                let mut cntr = 0;
                for n in &neighbors {
                    if rows[n.1][n.0] == 1 {
                        cntr += 1;
                    }
                }
                if cntr < 4 {
                    output += 1;
                }
            }
        }
    }
    output
}

fn part2(_rows: &[Vec<u8>]) -> u64 {
    let mut rows: Vec<Vec<u8>> = _rows.to_vec();
    let mut output = 0;

    loop {
        let mut remove_map: Vec<Vec<u32>> = vec![vec![0; rows[0].len()]; rows.len()];
        for (y, row) in rows.iter().enumerate() {
            for (x, p) in row.iter().enumerate() {
                if *p == 1 {
                    let neighbors = get_neighbors((x as i32, y as i32), &rows);
                    let mut cntr = 0;
                    for n in &neighbors {
                        if rows[n.1][n.0] == 1 {
                            cntr += 1;
                        }
                    }
                    if cntr < 4 {
                        // Mark for removal
                        remove_map[x][y] = 1;
                    }
                }
            }
        }

        if remove_map.iter().map(|r| r.iter().sum::<u32>()).sum::<u32>() == 0 {
            break
        }

        // Remove
        for (y, row) in remove_map.iter().enumerate() {
            for (x, p) in row.iter().enumerate() {
                if *p == 1 {
                    rows[x][y] = 0;
                    output += 1;
                }
            }
        }
    }
    output
}

fn main() {
    let map = parse("inputs/day04a.txt");
    let answer1 = part1(&map);
    let answer2 = part2(&map);
    println!("Examples:");
    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);

    let map = parse("inputs/day04.txt");
    let answer1 = part1(&map);
    let answer2 = part2(&map);
    println!("Challenges:");
    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}
