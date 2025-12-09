use std::io::Read;


fn parse(filename: &str) -> Vec<[i64; 3]> {
    let mut file = std::fs::File::open(filename).expect("failed to open file");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("failed to read file to string");

    s.trim().lines().map(|line| line.split(',').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>().try_into().unwrap()).collect()
}

fn euclid_dist_squared(p1: [i64; 3], p2: [i64; 3]) -> i64 {
    p1.iter().zip(p2.iter()).map(|(t1, t2)| (t1 - t2).pow(2)).sum()
}

fn connect_circuits(i1: usize, i2: usize, circuits: &mut Vec<Vec<usize>>) {
    let mut i1_circuit_idx = None;
    let mut i2_circuit_idx = None;
    for (circ_idx, circuit) in circuits.iter().enumerate() {
        for node in circuit {
            if *node == i1 {
                i1_circuit_idx = Some(circ_idx);
            }
            if *node == i2 {
                i2_circuit_idx = Some(circ_idx);
            }
        }
    }

    if i1_circuit_idx.is_none() && i2_circuit_idx.is_none() {
        // New circuit from isolated junctions
        circuits.push(vec![i1, i2]);
    } else if i1_circuit_idx.is_none() {
        // i2 in circuit already, i1 isolated
        circuits[i2_circuit_idx.unwrap()].push(i1);
    } else if i2_circuit_idx.is_none() {
        // i1 in circuit already, i2 isolated
        circuits[i1_circuit_idx.unwrap()].push(i2);
    } else {
        // Both indexes already in circuits, join them if they're not already
        if i1_circuit_idx != i2_circuit_idx {
            let mut tmp = circuits[i2_circuit_idx.unwrap()].clone();
            circuits[i1_circuit_idx.unwrap()].append(&mut tmp);
            circuits.remove(i2_circuit_idx.unwrap());
        }
    }
}

fn part1(tup: &[[i64; 3]], n_connections: usize) -> usize {
    let mut dists: Vec<(usize, usize, i64)> = vec![];
    for p1idx in 0..tup.len() {
        let p1 = tup[p1idx];
        for p2idx in p1idx + 1..tup.len() {
            let p2 = tup[p2idx];
            dists.push((p1idx, p2idx, euclid_dist_squared(p1, p2)));
        }
    }

    // Sort the pairs by the distances
    dists.sort_by(|a, b| a.2.cmp(&b.2));

    // Make connections, keep track of circuits
    let mut circuits: Vec<Vec<usize>> = vec![];
    for i in 0..n_connections {
        let (i1, i2, _d) = dists[i];
        connect_circuits(i1, i2, &mut circuits);
    }

    // Sort circuits by size
    circuits.sort_by(|a, b| a.len().cmp(&b.len()));
    circuits.into_iter().rev().take(3).map(|c| c.len()).product()
}

fn part2(tup: &[[i64; 3]]) -> i64 {
    let mut dists: Vec<(usize, usize, i64)> = vec![];
    for p1idx in 0..tup.len() {
        let p1 = tup[p1idx];
        for p2idx in p1idx + 1..tup.len() {
            let p2 = tup[p2idx];
            dists.push((p1idx, p2idx, euclid_dist_squared(p1, p2)));
        }
    }

    // Sort the pairs by the distances
    dists.sort_by(|a, b| a.2.cmp(&b.2));

    // Make connections, keep track of circuits
    let mut circuits: Vec<Vec<usize>> = vec![];
    for i in 0..dists.len() {
        let (i1, i2, _d) = dists[i];
        connect_circuits(i1, i2, &mut circuits);
        if circuits.len() == 1 && circuits[0].len() == tup.len() {
            return tup[i1][0] * tup[i2][0];
        }
    }
    0
}

fn main() {
    let tup = parse("inputs/day08a.txt");
    let answer1 = part1(&tup, 10);
    let answer2 = part2(&tup);
    println!("Examples:");
    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);

    let tup = parse("inputs/day08.txt");
    let answer1 = part1(&tup, 1000);
    let answer2 = part2(&tup);
    println!("Challenges:");
    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}
