use std::collections::HashSet;
use std::io::Read;
use std::time::Instant;

type Input = (usize, u64, Vec<u64>, Vec<u64>);

fn parse(filename: &str) -> Vec<Input> {
    let mut file = std::fs::File::open(filename).expect("failed to open file");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("failed to read file to string");

    s.trim().lines().map(|line| {
        // First part indicator lights in [...]
        let lights: String = line.split('[').skip(1).take(1).collect();
        let lights = lights.split(']').next().unwrap();
        let nlights = lights.len();
        let mut lightint = 0_u64;
        for (i, c) in lights.chars().enumerate() {
            match c {
                '#' => lightint ^= 1_u64 << i,
                _ => (),
            };
        }

        // Then parse wiring button schematics in (...)
        let schematic: String = line.split(']').skip(1).collect();
        let schematic = schematic.split('{').next().unwrap().trim();
        let mut buttons = vec![];
        let siter = schematic.split(&['(', ')', ' ']);
        for tmp in siter {
            if tmp.len() > 0 {
                let mut buttonint = 0_u64;
                for i in tmp.split(',') {
                    buttonint ^= 1_u64 << i.parse::<u64>().unwrap();
                }
                buttons.push(buttonint);
            }
        }

        // Finally parse joltage requirements in {...}
        let joltage: String = line.split('{').skip(1).collect();
        let joltage = joltage.split('}').next().unwrap().trim();
        let joltages: Vec<u64> = joltage.split(',').map(|j| j.parse::<u64>().unwrap()).collect();

        (nlights, lightint, buttons, joltages)
    }).collect::<Vec<_>>()
}

fn build_state_transition_table(state_length: usize, buttons: &[u64]) -> Vec<Vec<u64>> {
    let n = 2_usize.pow(state_length as u32);
    (0..n).map(|i| {
        buttons.iter().map(|b| {
            // What happens when we're in state i and press button b?
            i as u64 ^ b
        }).collect()
    }).collect()
}

// Do BFS on state graph. Each state is a node in a graph, we have the transition matrix which
// tells us how to move from one node to any neighbor in the graph. Start at state = 0, BFS until
// we get to the final state, record the pathlength.
fn bfs(start: u64, end: u64, state_transitions: &[Vec<u64>]) -> usize {
    let mut visited = HashSet::new();
    let mut frontier = HashSet::new();
    frontier.insert(start);
    let mut n_steps = 0;
    while frontier.len() > 0 {
        let mut next_frontier = HashSet::new();
        for p in &frontier {
            let neighbors = &state_transitions[*p as usize];
            for candidate in neighbors {
                if *candidate == end {
                    return n_steps + 1;
                }
                if !visited.contains(candidate) {
                    // Verify point _next_ to something in lines
                    next_frontier.insert(*candidate);
                }
            }
        }
        for p in frontier {
            visited.insert(p);
        }
        frontier = next_frontier;
        n_steps += 1;
    }
    let mut visited = visited.into_iter().collect::<Vec<u64>>();
    visited.sort();
    println!("visited: {:?}", visited);
    panic!("Somethings broken, no path to end");
}

fn part1(machines: &[Input]) -> usize {
    let mut output = 0;
    for m in machines {
        // transitions[state][button_idx] -> next_state
        let transitions = build_state_transition_table(m.0, &m.2);
        let pathlength = bfs(0, m.1, &transitions);
        output += pathlength;
    }
    output
}

fn print_matrix(m: &[Vec<i64>]) {
    for row in 0..m.len() {
        println!("{:?}", m[row]);
    }
}

fn pivot(m: &mut [Vec<i64>]) {
    let nrows = m.len();
    let endlen = nrows - 2;
    let ncols = m[0].len();
    let num_buttons = ncols - 3 - endlen;

    println!("pivot start");
    print_matrix(&m);
    println!();

    let mut cntr = 0;
    loop {
        // Scan columns, then rows looking for a positive element
        let mut col = 2;
        for c in 2..2+num_buttons {
        //for c in 2..ncols-1 {
            if m[0][c] > 0 {
                col = c;
                break;
            }
        }

        // We have chosen a col to pivot on
        println!("col: {:?}", col);

        // Given the column, choose the best row that minimizes b term
        let mut min = f64::MAX;
        let mut minidx = 0;
        for i in 0..endlen {
            let pivot_element = m[2+i][col];
            if pivot_element > 0 {
                let bterm = m[2+i][ncols-1] as f64 / pivot_element as f64;
                if bterm < min {
                    min = bterm;
                    minidx = i;
                }
            }
        }

        // `minidx` is our pivot row, m[2+minidx][col] is our pivot element, and it
        // should always be `1`.
        println!("minidx: {}", minidx);

        // Walk the other rows, zero them out
        for i in 0..nrows {
            if i != minidx + 2 {
                let mut a = m[i][col];
                let mut multiplier = true;
                if a % m[2+minidx][col] == 0 {
                    a /= m[2+minidx][col];
                    multiplier = false;
                }
                for j in 0..ncols {
                    if multiplier {
                        m[i][j] *= m[2+minidx][col];
                    }
                    m[i][j] -= a * m[2+minidx][j];
                }
            }
        }

        // Walk rows, if everything divisible by 2, 3, 5, or 10 just do it
        for i in 0..nrows {
            let mut all_5 = true;
            for x in &m[i] {
                if *x % 5 != 0 {
                    all_5 = false;
                    break;
                }
            }
            if all_5 {
                for j in 0..ncols {
                    m[i][j] /= 5;
                }
            }
        }

        print_matrix(&m);
        println!();

        cntr += 1;

        // When artificial variables are 0'd we're done
        if m[0][ncols-1] == 0 || cntr > num_buttons {
            break;
        }
    }
}

// Linear programming. Simplex method.
fn simplex(end: &[u64], buttons: &[u64]) -> i64 {
    // Augmented tableau form
    let nrows = 2 + end.len();
    let ncols = buttons.len() + end.len() + 3;
    let mut m = vec![vec![0_i64; ncols]; nrows];
    m[0][0] = 1;
    for i in 0..end.len() {
        m[0][buttons.len() + 2 + i] = -1;
    }
    m[1][1] = 1;
    for i in 0..buttons.len() {
        m[1][2 + i] = -1;
    }

    // Now fill out the buttons as columns starting row 3, col 3
    for j in 0..buttons.len() {
        for i in 0..end.len() {
            m[2+i][2+j] = ((buttons[j] >> i) & 1) as i64;
        }
    }

    // Identity to the right of the buttons
    for i in 0..end.len() {
        m[2+i][2+buttons.len()+i] = 1;
    }

    // End state desired last column on the right
    for i in 0..end.len() {
        m[2+i][2+buttons.len()+end.len()] = end[i] as i64;
    }

    print_matrix(&m);
    println!();

    // Add button rows to artificial objective, row 1
    for i in 0..end.len() {
        for j in 0..ncols {
            m[0][j] += m[2+i][j];
        }
    }

    println!("Pre-pivot");
    print_matrix(&m);
    println!();

    // Run pivoting algorithm until artificial objective is 0'd out
    pivot(&mut m);
    print_matrix(&m);
    println!();

    // Simplify down to equivalent canonical tableau
    m.remove(0);
    for _ in 0..end.len() {
        for row in 0..m.len() {
            m[row].remove(2+buttons.len());
        }
    }
    for row in 0..m.len() {
        m[row].remove(0);
    }
    let nrows = m.len();
    let ncols = m[0].len();

    println!("Simplified");
    print_matrix(&m);
    println!();

    // This isn't guaranteed optimal yet, no?
    for c in 1..ncols-1 {
        if m[0][c] > 0 {
            // solve this column?
            // Given the column, choose the best row that minimizes b term
            let mut min = f64::MAX;
            let mut minidx = 0;
            for i in 1..nrows {
                let pivot_element = m[i][c];
                if pivot_element > 0 {
                    let bterm = m[i][ncols-1] as f64 / pivot_element as f64;
                    if bterm < min {
                        min = bterm;
                        minidx = i;
                    }
                }
            }

            // Walk the other rows, zero them out
            for i in 0..nrows {
                if i != minidx {
                    let a = m[i][c];
                    for j in 0..ncols {
                        m[i][j] *= m[minidx][c];
                        m[i][j] -= a * m[minidx][j];
                    }
                }
            }

            print_matrix(&m);
            println!();
        }
    }

    // TODO: We're hitting this assert, why?
    assert_eq!(m[0][ncols-1]%m[0][0], 0);

    // Read off row 2, far right as # steps
    m[0][ncols-1] / m[0][0]
}

// 21422 is too low
fn part2(machines: &[Input]) -> usize {
    let mut output = 0;
    for m in machines {
        println!("{:?}", m);

        let pathlength = simplex(&m.3, &m.2);

        println!("pathlength: {}", pathlength);
        println!();
        output += pathlength;
    }
    output as usize
}

fn main() {
    let t0 = Instant::now();

    println!("Examples:");
    let machines = parse("inputs/day10a.txt");
    let answer1 = part1(&machines);
    println!("Part 1: {}", answer1);
    let answer2 = part2(&machines);
    println!("Part 2: {}", answer2);

    //panic!("asdf");

    let machines = parse("inputs/day10.txt");
    let answer1 = part1(&machines);
    let answer2 = part2(&machines);
    println!("Challenges:");
    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);

    println!("Time: {} ms", 1000.0 * t0.elapsed().as_secs_f64());
}
