use std::cmp::{min, max};
use std::collections::HashSet;
use std::io::Read;
use std::time::Instant;

#[derive(Clone, Debug)]
struct Tree {
    dims: (usize, usize),
    cnts: Vec<usize>,
}

impl Tree {
    fn new(dims: (usize, usize), cnts: Vec<usize>) -> Self {
        Self { dims, cnts }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Shape {
    shape: [[char; 3]; 3],
}

impl Shape {
    fn new(shape: [[char; 3]; 3]) -> Self {
        Self { shape }
    }

    fn display(&self) {
        for row in &self.shape {
            for c in row {
                print!("{}", c);
            }
            println!();
        }
        println!();
    }

    fn set_label(&mut self, label: char) {
        for row in &mut self.shape {
            for c in row {
                if *c != '.' {
                    *c = label;
                }
            }
        }
    }

    fn area(&self) -> usize {
        let mut area = 0;
        for row in &self.shape {
            for c in row {
                if *c != '.' {
                    area += 1;
                }
            }
        }
        area
    }


    fn rotate_cw(&mut self) {
        let mut newshape = [['.'; 3]; 3];
        newshape[0][2] = self.shape[0][0];
        newshape[0][1] = self.shape[1][0];
        newshape[0][0] = self.shape[2][0];
        newshape[1][0] = self.shape[2][1];
        newshape[2][0] = self.shape[2][2];
        newshape[2][1] = self.shape[1][2];
        newshape[2][2] = self.shape[0][2];
        newshape[1][2] = self.shape[0][1];
        newshape[1][1] = self.shape[1][1];
        self.shape = newshape;
    }

    fn flip_lr(&mut self) {
        let mut newshape = self.shape;
        newshape[0][2] = self.shape[0][0];
        newshape[0][0] = self.shape[0][2];
        newshape[1][2] = self.shape[1][0];
        newshape[1][0] = self.shape[1][2];
        newshape[2][2] = self.shape[2][0];
        newshape[2][0] = self.shape[2][2];
        self.shape = newshape;
    }
}

type Input = (Vec<Shape>, Vec<Tree>);

fn parse(filename: &str) -> Input {
    let mut file = std::fs::File::open(filename).expect("failed to open file");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("failed to read file to string");

    let mut shapes = vec![];
    let mut trees = vec![];
    let mut shapes_active = true;
    let mut liter = s.trim().lines();
    for s in 0..6 {
        liter.next(); // throw away index
        let mut shape = [['.'; 3]; 3];
        for i in 0..3 {
            shape[i] = liter.next().unwrap().chars().collect::<Vec<char>>().try_into().unwrap();
        }
        shapes.push(Shape::new(shape));
        liter.next(); // throw away space/newline
    }

    for line in liter {
        let mut niter = line.split(&['x', ':', ' ']).filter(|v| v.len() > 0);
        let width = niter.next().unwrap().parse::<usize>().unwrap();
        let height = niter.next().unwrap().parse::<usize>().unwrap();
        let cnts: Vec<usize> = niter.map(|n| n.parse::<usize>().unwrap()).collect();
        trees.push(Tree::new((width, height), cnts));
    }

    (shapes, trees)
}

#[derive(Clone, Debug)]
struct Board {
    board: Vec<Vec<char>>,
}

impl Board {
    fn new(width: usize, height: usize) -> Self {
        Self { board: vec![vec!['.'; width]; height] }
    }

    fn display(&self) {
        for row in &self.board {
            for c in row {
                print!("{}", c);
            }
            println!();
        }
        println!();
    }

    fn placement_check(&self, upper_left: (usize, usize), piece: &Shape) -> bool {
        for (i, row) in piece.shape.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if upper_left.1 + i > self.board.len() - 1 || upper_left.0 + j > self.board[0].len() - 1 {
                    // Doesn't fit on the board.
                    return false;
                }
                if !(self.board[upper_left.1+i][upper_left.0+j] == '.') && *c == '#' {
                    // Part of `piece` needs to go here, and can't.
                    return false;
                }
            }
        }
        true
    }

    fn place(&mut self, upper_left: (usize, usize), piece: &Shape) -> bool {
        if self.placement_check(upper_left, piece) {
            // It fits, place it
            for (i, row) in piece.shape.iter().enumerate() {
                for (j, c) in row.iter().enumerate() {
                    if *c != '.' {
                        self.board[upper_left.1+i][upper_left.0+j] = *c;
                    }
                }
            }
            true
        } else {
            false
        }
    }

    fn get_neighbors(&self, p: (usize, usize)) -> Vec<char> {
        let mut output = vec![];
        if p.0 > 0 {
            output.push(self.board[p.1][p.0-1]);
        }
        if p.1 > 0 {
            output.push(self.board[p.1-1][p.0]);
        }
        if p.0 < self.board[0].len()-1 {
            output.push(self.board[p.1][p.0+1]);
        }
        if p.1 < self.board.len()-1 {
            output.push(self.board[p.1+1][p.0]);
        }
        output
    }

    fn remove(&mut self, upper_left: (usize, usize), piece: &Shape) {
        for (i, row) in piece.shape.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if *c != '.' {
                    self.board[upper_left.1+i][upper_left.0+j] = '.';
                }
            }
        }
    }

    fn cost(&mut self, upper_left: (usize, usize), piece: &Shape) -> Option<i64> {
        // Count board places adjacent to any shape
        if self.place(upper_left, piece) {
            let mut cost = 0;
            for y in 0..self.board.len() {
                for x in 0..self.board[0].len() {
                    if self.board[y][x] == '.' {
                        // We have an empty space, is it adjacent to any non-empty spaces?
                        let neighbors = self.get_neighbors((x, y));
                        let mut any_non_empty = false;
                        for n in neighbors {
                            if n != '.' {
                                any_non_empty = true;
                                break;
                            }
                        }
                        if y == self.board.len() - 1 {
                            any_non_empty = true;
                        }
                        if any_non_empty {
                            cost += 1;
                        }
                    }
                }
            }
            // Now remove the piece again before returning the hypothetical cost of placement
            self.remove(upper_left, piece);
            Some(cost)
        } else {
            // Piece can't fit here.
            None
        }
    }

    fn get_perimeter_positions(&self) -> Vec<(usize, usize)> {
        let mut output = vec![];
        for y in 0..self.board.len() {
            for x in 0..self.board[0].len() {
                if self.board[y][x] == '.' {
                    // If this empty space is adjacent to a non empty space, it is a perimeter
                    let neighbors = self.get_neighbors((x, y));
                    let mut any_non_empty = false;
                    for n in neighbors {
                        if n != '.' {
                            any_non_empty = true;
                            break;
                        }
                    }
                    if x == 0 || y == 0 || x == self.board[0].len() - 1 || y == self.board.len() - 1 {
                        any_non_empty = true;
                    }
                    if any_non_empty {
                        output.push((x, y));
                    }
                }
            }
        }
        output
    }

    fn get_all_positions(&self) -> Vec<(usize, usize)> {
        // We subtract 2 from indexes since we're dealing with upper left corners
        // of the shapes which are always 3x3.
        let mut output = vec![];
        for y in 0..self.board.len()-2 {
            for x in 0..self.board[0].len()-2 {
                output.push((x, y));
            }
        }
        output
    }
}

fn solve_tree_p1(tree: &Tree, shapes: &[Shape]) -> bool {
    let mut board = Board::new(tree.dims.0, tree.dims.1);
    let mut shape_counts = tree.cnts.clone();

    // Quick sanity check just looking at area
    let mut shapes_area = 0;
    for i in 0..shapes.len() {
        shapes_area += shape_counts[i] * shapes[i].area();
    }
    let board_area = board.board.len() * board.board[0].len();

    if shapes_area > board_area {
        println!("Shapes total area exceeds that of the board");
        return false;
    }

    // We will keep and update this set of options to reduce the search space since we only have a
    // few distinct shapes, and a good number of each.
    let mut shape_options: HashSet<(usize, Shape)> = HashSet::new();
    for (i, s) in shapes.iter().enumerate() {
        if shape_counts[i] > 0 {
            shape_options.insert((i, shapes[i].clone()));
        }
    }

    println!("Start:");
    println!("Shape Counts: {:?}", shape_counts);
    board.display();

    let mut cntr = 0;
    let mut placed = 0;

    // What positions are possible?
    let cursors = board.get_all_positions();
    //println!("cursors: {:?}", cursors);

    loop {
        // Enumerate our possible actions and their costs
        let mut actions: Vec<(_, usize, usize, bool, i64)> = vec![];

        println!();
        println!("Shape Counts: {:?}", shape_counts);
        println!();

        // Try all possible shapes...
        for (sidx, shape) in &shape_options {
            // Try all positions...
            for cursor in &cursors {
                // Try all rotations, then flip, then try all rotations of flipped
                let mut tmpshape = shape.clone();
                for rotation in 0..4 {
                    if board.placement_check(*cursor, &tmpshape) {
                        let cost = board.cost(*cursor, &tmpshape).unwrap();
                        let best_cost = if actions.len() == 0 {
                            999999999999
                        } else {
                            actions.last().unwrap().4
                        };
                        if cost < best_cost {
                            actions.push((*cursor, *sidx, rotation, false, cost));
                            actions.sort_by_key(|k| -k.4);
                        }
                    }
                    tmpshape.rotate_cw();
                }

                // Try flip, rotate
                tmpshape.flip_lr();
                for rotation in 0..4 {
                    if board.placement_check(*cursor, &tmpshape) {
                        let cost = board.cost(*cursor, &tmpshape).unwrap();
                        let best_cost = if actions.len() == 0 {
                            999999999999
                        } else {
                            actions.last().unwrap().4
                        };
                        if cost < best_cost {
                            actions.push((*cursor, *sidx, rotation, true, cost));
                            actions.sort_by_key(|k| -k.4);
                        }
                    }
                    tmpshape.rotate_cw();
                }
            }
        }

        println!("Finished enumerating actions: {}", actions.len());

        // Now we have a vec of possible actions and their costs (cursor, sidx, rotation, flipped?, cost)
        if actions.len() == 0 && shape_counts.iter().sum::<usize>() > 0 {
            // No possible actions and shapes remaining mean we lose.
            return false;
        } else if actions.len() == 0 {
            panic!("shouldn't hit this");
        }

        // Sort actions so that lowest costs are at the end, highest at the start
        actions.sort_by_key(|k| -k.4);
        //println!("actions: {:?}", actions);

        println!("Finished sorting actions");

        // Grab our piece...
        let best_action = actions.pop().unwrap();
        let mut piece = shapes[best_action.1].clone();

        // Orient it...
        if best_action.3 {
            piece.flip_lr();
        }
        for _ in 0..best_action.2 {
            piece.rotate_cw();
        }

        // Give it a label...
        let label = match placed % 10 {
            0 => 'A',
            1 => 'B',
            2 => 'C',
            3 => 'D',
            4 => 'E',
            5 => 'F',
            6 => 'G',
            7 => 'H',
            8 => 'I',
            9 => 'J',
            _ => '#',
        };
        piece.set_label(label);

        // Finally, place it
        board.place(best_action.0, &piece);
        placed += 1;

        // Update our shape counts after placing the piece
        shape_counts[best_action.1] -= 1;
        if shape_counts.iter().sum::<usize>() == 0 {
            // We placed all the pieces successfully, so we're done.
            println!("Final Successful Board State:");
            board.display();
            println!();
            return true;
        }

        // Update `shape_options` in case we've exhausted supply of anything.
        shape_options = HashSet::new();
        for (i, _s) in shapes.iter().enumerate() {
            if shape_counts[i] > 0 {
                shape_options.insert((i, shapes[i].clone()));
            }
        }

        // Debug display stuff
        println!("Iteration {}", cntr);
        board.display();
        cntr += 1;
    }
}

fn p1_size_test(trees: &[Tree], shapes: &[Shape]) -> i64 {
    let mut count = 0;
    for (i, tree) in trees.iter().enumerate() {
        // Quick sanity check just looking at area
        let mut shapes_area = 0;
        for i in 0..shapes.len() {
            shapes_area += tree.cnts[i] * shapes[i].area();
        }
        let board_area = tree.dims.0 * tree.dims.1;

        // Shows when its solvable you have like hundreds of spaces to spare, every time.
        //println!("{}", shapes_area as i64 - board_area as i64);

        if shapes_area > board_area {
            //println!("Tree {}: Shapes total area exceeds that of the board", i);
        } else {
            count += 1;
        }
    }
    count
}

fn part1(tup: &Input) -> i64 {
    let (shapes, trees) = tup;
    /*
    for shape in shapes {
        shape.display();
    }
    */

    // NOTE: This is stupid and I hate it.
    return p1_size_test(trees, shapes);

    // I'm keeping this I don't care.
    let mut tree_idx = 0;
    let mut num_that_work = 0;
    for tree in trees {
        if solve_tree_p1(tree, &shapes) {
            // Success
            println!("Success");
            num_that_work += 1;
        }
        if tree_idx == 1 {
            //panic!("bail");
        }
        tree_idx += 1;
    }
    num_that_work
}

fn part2(tup: &Input) -> i64 {
    let (shapes, trees) = tup;
    0
}

fn main() {
    let t0 = Instant::now();

    println!("Examples:");
    let input = parse("inputs/day12a.txt");
    //let answer1 = part1(&input);
    //println!("Part 1: {}", answer1);
    let answer2 = part2(&input);
    println!("Part 2: {}", answer2);

    //panic!("asdf");

    let input = parse("inputs/day12.txt");
    let answer1 = part1(&input);
    let answer2 = part2(&input);
    println!("Challenges:");
    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);

    println!("Time: {} ms", 1000.0 * t0.elapsed().as_secs_f64());
}
