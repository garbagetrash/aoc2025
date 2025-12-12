use std::collections::{HashMap, HashSet};
use std::io::Read;
use std::time::Instant;

type Input = HashMap<String, Vec<String>>;

fn parse(filename: &str) -> Input {
    let mut file = std::fs::File::open(filename).expect("failed to open file");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("failed to read file to string");

    let mut output: Input = HashMap::new();
    for line in s.trim().lines() {
        let mut liter = line.split(&[':', ' ']).map(|t| t.trim());
        let name = liter.next().unwrap();
        let outputs: Vec<String> = liter.map(|r| r.to_string()).filter(|s| s.len() > 0).collect();
        output.insert(name.to_string(), vec![]);
        for o in &outputs {
            if let Some(v) = output.get_mut(name) {
                (*v).push(o.to_string());
            }
        }
    }
    output
}

fn all_nodes(connections: &Input) -> HashSet<String> {
    let mut output: HashSet<String> = HashSet::new();
    for (node, downstream) in connections {
        output.insert(node.to_string());
        for d in downstream {
            output.insert(d.to_string());
        }
    }
    output
}

fn count_ways_to_nodes(start: &str, connections: &Input) -> HashMap<String, usize> {
    let rev_connections = reverse_hashmap(connections);
    let mut ways_to_node: HashMap<String, usize> = HashMap::new();
    ways_to_node.insert(start.to_string(), 1);
    let mut unsolved = all_nodes(connections);
    unsolved.remove(start);
    let mut zeroed = vec![];
    for u in &unsolved {
        if !rev_connections.contains_key(u) {
            zeroed.push(u.to_string());
        }
    }
    for z in &zeroed {
        unsolved.remove(z);
        ways_to_node.insert(z.to_string(), 0);
    }

    // loop over nodes building out dependency tree, only enter into ways_to_node once all deps
    // are there.
    loop {
        let mut to_remove = vec![];
        for node in &unsolved {
            if let Some(upstream) = rev_connections.get(node) {
                let mut all_deps = true;
                for dep in upstream {
                    if unsolved.contains(dep) {
                        all_deps = false;
                        break;
                    }
                }
                if all_deps {
                    // Solve node
                    let value = upstream.iter().map(|n| ways_to_node.get(n).unwrap()).sum::<usize>();
                    ways_to_node.insert(node.to_string(), value);
                    to_remove.push(node.to_string());
                }
            }
        }
        for tr in &to_remove {
            unsolved.remove(tr);
        }
        if unsolved.len() == 0 {
            break;
        }
    }
    ways_to_node
}

fn reverse_hashmap(map: &Input) -> Input {
    let mut output: Input = HashMap::new();
    for c in map {
        for name in c.1 {
            if let Some(v) = output.get_mut(name) {
                (*v).push(c.0.to_string());
            } else {
                output.insert(name.to_string(), vec![c.0.to_string()]);
            }
        }
    }
    output
}

fn part1(map: &Input) -> usize {
    let ways_to_nodes: HashMap<String, usize> = count_ways_to_nodes("you", map);
    *ways_to_nodes.get("out").unwrap()
}

fn part2(map: &Input) -> usize {
    let ways_to_nodes: HashMap<String, usize> = count_ways_to_nodes("svr", map);
    let svr_fft = ways_to_nodes.get("fft").unwrap();

    let ways_to_nodes: HashMap<String, usize> = count_ways_to_nodes("fft", map);
    let fft_dac = ways_to_nodes.get("dac").unwrap();

    let ways_to_nodes: HashMap<String, usize> = count_ways_to_nodes("dac", map);
    let dac_out = ways_to_nodes.get("out").unwrap();

    svr_fft * fft_dac * dac_out
}

fn main() {
    let t0 = Instant::now();

    println!("Examples:");
    let map = parse("inputs/day11a.txt");
    let answer1 = part1(&map);
    println!("Part 1: {}", answer1);
    let map = parse("inputs/day11b.txt");
    let answer2 = part2(&map);
    println!("Part 2: {}", answer2);

    println!("Challenges:");
    let map = parse("inputs/day11.txt");
    let answer1 = part1(&map);
    println!("Part 1: {}", answer1);
    let answer2 = part2(&map);
    println!("Part 2: {}", answer2);

    println!("Time: {} ms", 1000.0 * t0.elapsed().as_secs_f64());
}
