use std::collections::HashSet;
use std::io::Read;
use std::time::Instant;

type Input = (usize, u64, Vec<u64>, Vec<u64>);

fn parse(filename: &str) -> Vec<Input> {
    let mut file = std::fs::File::open(filename).expect("failed to open file");
    let mut s = String::new();
    file.read_to_string(&mut s)
        .expect("failed to read file to string");

    s.trim()
        .lines()
        .map(|line| {
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
            let joltages: Vec<u64> = joltage
                .split(',')
                .map(|j| j.parse::<u64>().unwrap())
                .collect();

            (nlights, lightint, buttons, joltages)
        })
        .collect::<Vec<_>>()
}

fn build_state_transition_table(state_length: usize, buttons: &[u64]) -> Vec<Vec<u64>> {
    let n = 2_usize.pow(state_length as u32);
    (0..n)
        .map(|i| {
            buttons
                .iter()
                .map(|b| {
                    // What happens when we're in state i and press button b?
                    i as u64 ^ b
                })
                .collect()
        })
        .collect()
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

fn print_matrix<T: std::fmt::Debug>(m: &[Vec<T>]) {
    for row in 0..m.len() {
        println!("{:?}", m[row]);
    }
}

// TODO: When pivoting, we sometime reuse a given row which reintroduces the first pivot element
//       from that row back into the objective function. This seems bad? But what else would we do
//       given # buttons ie columns > # counter ie rows?
fn pivot(m: &mut [Vec<i64>]) -> Vec<usize> {
    let nrows = m.len();
    let endlen = nrows - 2;
    let ncols = m[0].len();
    let num_buttons = ncols - 3 - endlen;

    //println!("pivot start");
    //print_matrix(&m);
    //println!();

    let mut pivot_columns = vec![];
    let mut cntr = 0;
    loop {
        // Scan columns, then rows looking for a positive element
        let mut col = 2;
        for c in 2..2 + num_buttons {
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
            let pivot_element = m[2 + i][col];
            if pivot_element > 0 {
                let bterm = m[2 + i][ncols - 1] as f64 / pivot_element as f64;
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
                if a % m[2 + minidx][col] == 0 {
                    a /= m[2 + minidx][col];
                    multiplier = false;
                }
                for j in 0..ncols {
                    if multiplier {
                        m[i][j] *= m[2 + minidx][col];
                    }
                    m[i][j] -= a * m[2 + minidx][j];
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

        //print_matrix(&m);
        //println!();

        pivot_columns.push(col);
        cntr += 1;

        // When artificial variables are 0'd we're done
        if m[0][ncols - 1] == 0 || cntr > num_buttons {
            break;
        }
    }
    pivot_columns.sort();
    pivot_columns
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
            m[2 + i][2 + j] = ((buttons[j] >> i) & 1) as i64;
        }
    }

    // Identity to the right of the buttons
    for i in 0..end.len() {
        m[2 + i][2 + buttons.len() + i] = 1;
    }

    // End state desired last column on the right
    for i in 0..end.len() {
        m[2 + i][2 + buttons.len() + end.len()] = end[i] as i64;
    }

    //print_matrix(&m);
    //println!();

    // Add button rows to artificial objective, row 1
    for i in 0..end.len() {
        for j in 0..ncols {
            m[0][j] += m[2 + i][j];
        }
    }

    //println!("Pre-pivot");
    //print_matrix(&m);
    //println!();

    // Run pivoting algorithm until artificial objective is 0'd out
    let pivot_columns = pivot(&mut m);
    print_matrix(&m);
    println!();

    // Simplify down to equivalent canonical tableau
    m.remove(0);
    for _ in 0..end.len() {
        for row in 0..m.len() {
            m[row].remove(2 + buttons.len());
        }
    }
    for row in 0..m.len() {
        m[row].remove(0);
    }
    let nrows = m.len();
    let ncols = m[0].len();

    // This isn't guaranteed optimal yet, no?
    for c in 1..ncols - 1 {
        if m[0][c] > 0 {
            // solve this column?
            // Given the column, choose the best row that minimizes b term
            let mut min = f64::MAX;
            let mut minidx = 0;
            for i in 1..nrows {
                let pivot_element = m[i][c];
                if pivot_element > 0 {
                    let bterm = m[i][ncols - 1] as f64 / pivot_element as f64;
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

    println!("Simplified");
    print_matrix(&m);
    println!();

    // At this point we have an LP solution, but is it integer?
    let mut non_integer = false;
    let mut first_non_integer_column = None;
    let mut non_integer_solution = None;
    for c in pivot_columns {
        let c = c - 1; // We do this because the simplification removes the first column.
        let r = (1..nrows).position(|x| m[x][c] != 0).unwrap() + 1;
        //println!("c: {}, r: {}", c, r);
        if m[r][ncols - 1] % m[r][c] != 0 {
            // Problem: We have a non-integer variable in our LP solution.
            let solution = m[r][ncols - 1] as f64 / m[r][c] as f64;
            println!("Found non-integer solution {}", solution);
            non_integer = true;
            first_non_integer_column = Some(c);
            non_integer_solution = Some(solution);
        }
    }

    // TODO: Start the branch and prune proceedure...
    if non_integer {
        // Pick a non-integer variable solution xi = k, constrain it in 2 new optimization problems
        // as: xi <= floor(k), and the other with xi >= ceiling(k).
        // Now solve these, see if integer, recurse, keep best integer solution, and once we've
        // exhausted all created solutions choose the best integer solution so far. This should be
        // our optimium.
        let xi = non_integer_solution.unwrap();
    }

    // Verify we have an _integer_ number of button presses...
    assert_eq!(m[0][ncols - 1] % m[0][0], 0);

    // Read off row 2, far right as # steps
    m[0][ncols - 1] / m[0][0]
}

fn ge_pivot(r: usize, c: usize, next_row_to: usize, m: &mut [Vec<f64>]) {
    let nrows = m.len();
    let ncols = m[0].len();

    // We like our pivot elements to be 1.0
    if m[r][c] != 1.0 {
        let a = m[r][c];
        for cc in 0..ncols {
            m[r][cc] *= 1.0 / a;
        }
    }

    // This zeros out column c in m
    for rr in 0..nrows {
        if rr != r {
            let a = m[rr][c];
            for cc in 0..ncols {
                m[rr][cc] -= a * m[r][cc];
            }
        }
    }

    // Now lets do a row swap (row r and row c) to get I on left
    if r != next_row_to {
        let tmp = m[r].clone();
        for cc in 0..ncols {
            m[r][cc] = m[next_row_to][cc];
        }
        for cc in 0..ncols {
            m[next_row_to][cc] = tmp[cc];
        }
    }
}

fn gaussian_elimination(mut m: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let nrows = m.len();
    let ncols = m[0].len();

    let mut start_row = 0;
    for c in 0..ncols {
        let mut pivot_row = None;
        for r in start_row..nrows {
            if m[r][c] != 0.0 {
                // Clean mod, pivot on this element
                pivot_row = Some(r);
                start_row += 1;
                break;
            }
        }
        if let Some(prow) = pivot_row {
            //println!("Pivot column: {}", c);
            //println!("Pivot row   : {}", prow);
            ge_pivot(prow, c, start_row - 1, &mut m);
            //print_matrix(&m);
            //println!();
        } else {
            // Have to nudge over another column and try again.
        }

        if start_row == nrows {
            // Solved pivots equal to matrix rank, now we're done
            break;
        }
    }

    // Clean up numerical nonsense a bit
    for r in 0..nrows {
        for c in 0..ncols {
            if (m[r][c] - m[r][c].round()).abs() < 1e-9 {
                m[r][c] = m[r][c].round();
            }
        }
    }

    /*
    // Remove all 0's rows
    let mut remove_rows = vec![];
    for r in 0..nrows {
        let mut all_zeros = true;
        for c in 0..ncols {
            if m[r][c] != 0.0 {
                all_zeros = false;
                break;
            }
        }
        if all_zeros {
            remove_rows.push(r);
        }
    }

    while let Some(r) = remove_rows.pop() {
        m.remove(r);
    }
    */

    //println!("Final State after Gaussian Elimination");
    //print_matrix(&m);
    //println!();

    m
}

fn create_ax_b(n: i64, buttons: &[u64], end: &[u64]) -> Vec<Vec<f64>> {
    // Set up the constraints, [ A | b ]
    let nrows = end.len() + 1;
    let ncols = buttons.len() + 1;
    let mut m = vec![vec![0_f64; ncols]; nrows];

    // Fill out the buttons (A) as columns from the left
    for j in 0..buttons.len() {
        for i in 0..end.len() {
            m[i][j] = ((buttons[j] >> i) & 1) as f64;
        }
    }

    // End state desired last column on the right
    for i in 0..end.len() {
        m[i][buttons.len()] = end[i] as f64;
    }

    // Add one final constraint to sum number of presses = n
    for i in 0..ncols-1 {
        m[nrows-1][i] = 1.0;
    }

    m[nrows-1][ncols-1] = n as f64;

    println!("buttons.len(): {}", buttons.len());
    println!("end.len()    : {}", end.len());
    println!("Ax = b:");
    print_matrix(&m);
    println!();

    m
}

fn solution_valid(x: &[f64]) -> bool {
    for xx in x {
        if *xx < 0.0 || (xx - xx.round()).abs() > 1e-4 {
            return false;
        }
    }
    true
}

fn solution_positive(x: &[f64]) -> bool {
    for xx in x {
        if *xx < 0.0 {
            return false;
        }
    }
    true
}

fn setup_least_squares(axb: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    // A is m x n
    // x is n x 1
    // b is m x 1

    // Calculate nxn matrix A'A
    let n = axb[0].len() - 1;
    let m = axb.len();
    let mut ata_atb = vec![vec![0.0; n+1]; n];
    for r in 0..n {
        for c in 0..n {
            for i in 0..m {
                ata_atb[r][c] += axb[i][r] * axb[i][c];
            }
        }
    }

    // Calculate nx1 vector A'b
    for r in 0..n {
        for i in 0..m {
            ata_atb[r][n] += axb[i][r] * axb[i][n];
        }
    }

    //println!("Augmented Form of A'Ax = A'b");
    //print_matrix(&ata_atb);
    //println!();

    // They were stored in augmented form in the first place
    ata_atb
}

fn solve(axb: Vec<Vec<f64>>) -> Vec<f64> {
    let mut x = vec![];
    let axb = gaussian_elimination(axb);
    let nrows = axb.len();
    let ncols = axb[0].len();
    for r in 0..nrows {
        x.push(axb[r][ncols-1]);
    }
    x
}

// 21422 is too low
fn part2(machines: &[Input]) -> usize {
    let mut output = 0;
    for (i, m) in machines.iter().enumerate() {
        println!("Problem {}/{}", i + 1, machines.len());
        println!("{:?}", m);

        let min_presses = *m.3.iter().max().unwrap() as i64;

        let mut n = min_presses;
        loop {
            print!("n: {}\r", n);
            let axb = create_ax_b(n, &m.2, &m.3);
            let axb = setup_least_squares(axb);
            let x = solve(axb);
            if solution_positive(&x) {
                println!("n: {}, x: {:?}", n, x);
            }
            if solution_valid(&x) {
                println!("# Presses: {}", n);
                output += n;
                break;
            }
            n += 1;
            if n > 200 {
                panic!("asdf");
            }
        }
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
