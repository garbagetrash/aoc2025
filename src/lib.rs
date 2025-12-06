use std::cmp::{min, max};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Range {
    start: usize,
    end: usize,
}

impl Range {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start: min(start, end), end: max(start, end) }
    }

    pub fn intersect(&self, other: Range) -> Option<Range> {
        if self.start > other.end {
            None
        } else if other.start > self.end {
            None
        } else {
            Some(Range::new(max(self.start, other.start), min(self.end, other.end)))
        }
    }

    pub fn union(&self, other: Range) -> Vec<Range> {
        if self.start > other.end {
            vec![other, *self]
        } else if other.start > self.end {
            vec![*self, other]
        } else {
            vec![Range::new(min(self.start, other.start), max(self.end, other.end))]
        }
    }

    pub fn len(&self) -> usize {
        self.end - self.start + 1
    }
}

pub fn union(ranges: &[Range]) -> Vec<Range> {
    let mut output = vec![];
    for r in ranges {
        let mut new = *r;
        let mut remove = vec![];
        for (i, o) in output.iter().enumerate() {
            if let Some(intersect) = new.intersect(*o) {
                new = new.union(*o)[0];
                remove.push(i);
            }
        }
        for rem in remove.iter().rev() {
            output.remove(*rem);
        }
        output.push(new);
    }
    output
}
