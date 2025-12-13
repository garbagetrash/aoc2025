use std::cmp::{max, min};
use std::collections::HashSet;
use std::io::Read;
use std::time::Instant;

fn parse(filename: &str) -> Vec<[i64; 2]> {
    let mut file = std::fs::File::open(filename).expect("failed to open file");
    let mut s = String::new();
    file.read_to_string(&mut s)
        .expect("failed to read file to string");

    s.trim()
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
                .try_into()
                .unwrap()
        })
        .collect()
}

fn part1(tup: &[[i64; 2]]) -> i64 {
    let mut rectangles = vec![];
    for i in 0..tup.len() {
        for j in i + 1..tup.len() {
            rectangles.push(Rectangle::new(tup[i], tup[j]));
        }
    }
    let mut sizes: Vec<_> = rectangles.into_iter().map(|r| r.size()).collect();
    sizes.sort();
    *sizes.last().unwrap()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Line {
    p1: [i64; 2],
    p2: [i64; 2],
}

impl Line {
    fn new(p1: [i64; 2], p2: [i64; 2]) -> Self {
        if p1[0] + p1[1] < p2[0] + p2[1] {
            Self { p1, p2 }
        } else {
            Self { p1: p2, p2: p1 }
        }
    }
    fn is_horizontal(&self) -> bool {
        self.p1[1] == self.p2[1]
    }
    fn contains_point(&self, point: [i64; 2]) -> bool {
        if self.is_horizontal() {
            point[1] == self.p1[1] && self.p1[0] <= point[0] && self.p2[0] >= point[0]
        } else {
            point[0] == self.p1[0] && self.p1[1] <= point[1] && self.p2[1] >= point[1]
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Rectangle {
    p1: [i64; 2], // upper left
    p2: [i64; 2], // lower right
}

impl Rectangle {
    fn new(p1: [i64; 2], p2: [i64; 2]) -> Self {
        let upper_left = [min(p1[0], p2[0]), min(p1[1], p2[1])];
        let lower_right = [max(p1[0], p2[0]), max(p1[1], p2[1])];

        Self {
            p1: upper_left,
            p2: lower_right,
        }
    }
    fn size(&self) -> i64 {
        (self.p2[0] - self.p1[0] + 1) * (self.p2[1] - self.p1[1] + 1)
    }
    fn contains_point(&self, p: [i64; 2]) -> bool {
        self.p1[0] <= p[0] && p[0] <= self.p2[0] && self.p1[1] <= p[1] && p[1] <= self.p2[1]
    }
}

fn get_neighbors(p: [i64; 2]) -> Vec<[i64; 2]> {
    let mut output = Vec::with_capacity(4);
    output.push([p[0] - 1, p[1]]);
    output.push([p[0] + 1, p[1]]);
    output.push([p[0], p[1] - 1]);
    output.push([p[0], p[1] + 1]);
    output
}

fn get_neighbors_diag(p: [i64; 2]) -> Vec<[i64; 2]> {
    let mut output = Vec::with_capacity(8);
    output.push([p[0] - 1, p[1]]);
    output.push([p[0] + 1, p[1]]);
    output.push([p[0], p[1] - 1]);
    output.push([p[0], p[1] + 1]);
    output.push([p[0] - 1, p[1] - 1]);
    output.push([p[0] + 1, p[1] + 1]);
    output.push([p[0] + 1, p[1] - 1]);
    output.push([p[0] - 1, p[1] + 1]);
    output
}

fn walk_perimeter(start: [i64; 2], lines: &[Line]) -> HashSet<[i64; 2]> {
    let mut visited = HashSet::new();
    let mut frontier = HashSet::new();
    frontier.insert(start);
    while frontier.len() > 0 {
        //println!("frontier: {:?}", frontier);
        let mut next_frontier = HashSet::new();
        for p in &frontier {
            let n = get_neighbors(*p);
            //println!("n: {:?}", n);
            for candidate in n {
                //println!("candidate: {:?}", candidate);
                if !visited.contains(&candidate) {
                    let mut allowed = true;
                    // Verify point not in lines
                    for l in lines {
                        if l.contains_point(candidate) {
                            allowed = false;
                            break;
                        }
                    }
                    /*
                    if !allowed {
                        break;
                    }
                    */
                    // Verify point _next_ to something in lines
                    let mut next_to = false;
                    let nn = get_neighbors_diag(candidate);
                    for nnn in nn {
                        for l in lines {
                            if l.contains_point(nnn) {
                                next_to = true;
                                break;
                            }
                        }
                        if next_to {
                            break;
                        }
                    }
                    if allowed && next_to {
                        next_frontier.insert(candidate);
                    }
                }
            }
        }
        for p in frontier {
            visited.insert(p);
        }
        frontier = next_frontier;
    }
    visited
}

// Solution sketch:
// 1.) Pick upper left corner. Tile to left of this is guaranteed to be outside edge of mass.
// 2.) Walk along the perimeter by using get_neighbors, and throwing away any neighbor point that
//     isn't adjacent to some line segment (ie call get_neighbors on each neighbor and verify at
//     least one of those points is contained in some line segment.)
// 3.) Now that we have a... HashSet? Of perimeter points, start with each corner from input, and
//     'grow' a rectangle until it hits a perimeter point to the right, and one beneath.
// 4.) Take maximum sized grown rectangle. Done.

fn part2(tup: &[[i64; 2]]) -> i64 {
    // Create lines
    let mut lines = vec![];
    for i in 0..tup.len() - 1 {
        lines.push(Line::new(tup[i], tup[i + 1]));
    }
    lines.push(Line::new(tup[0], tup[tup.len() - 1]));

    // 1.) Pick upper left corner. Tile to left of this is guaranteed to be outside edge of mass.
    let mut ymin = tup[0][1];
    for t in tup {
        if t[1] < ymin {
            ymin = t[1];
        }
    }
    let mut xmin = tup[0][0];
    for t in tup {
        if t[1] == ymin && t[0] < xmin {
            xmin = t[0];
        }
    }
    let upper_left_perimeter_tile = [xmin - 1, ymin];

    // 2.) Walk along the perimeter by using get_neighbors, and throwing away any neighbor point that
    //     isn't adjacent to some line segment (ie call get_neighbors on each neighbor and verify at
    //     least one of those points is contained in some line segment.)
    let perimeter = walk_perimeter(upper_left_perimeter_tile, &lines);

    // 3.) Now that we have a... HashSet? Of perimeter points, start with each corner from input, and
    //     'grow' a rectangle until it hits a perimeter point to the right, and one beneath.
    let mut rectangles = vec![];
    for i in 0..tup.len() {
        for j in i + 1..tup.len() {
            let r = Rectangle::new(tup[i], tup[j]);
            let mut allowed = true;
            for p in &perimeter {
                if r.contains_point(*p) {
                    allowed = false;
                    break;
                }
            }
            if allowed {
                rectangles.push(r);
            }
        }
    }

    // 4.) Take maximum sized grown rectangle. Done.
    let mut sizes: Vec<i64> = rectangles.into_iter().map(|r| r.size()).collect();
    sizes.sort();
    *sizes.last().unwrap()
}

fn main() {
    let t0 = Instant::now();

    println!("Examples:");
    let tup = parse("inputs/day09a.txt");
    let answer1 = part1(&tup);
    println!("Part 1: {}", answer1);
    let answer2 = part2(&tup);
    println!("Part 2: {}", answer2);

    let tup = parse("inputs/day09.txt");
    let answer1 = part1(&tup);
    let answer2 = part2(&tup);
    println!("Challenges:");
    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);

    println!("Time: {} ms", 1000.0 * t0.elapsed().as_secs_f64());
}
