use std::collections::HashMap;

pub fn solve_part1(input: &str) -> i32 {
    let lines = parse_input(input)
        .iter()
        .filter(|l| l.line_type().part1_type())
        .copied()
        .collect::<Vec<_>>();

    let mut cross_counter: HashMap<(i32, i32), i32> = HashMap::new();
    for i in 0..(lines.len() - 1) {
        for j in (i + 1)..lines.len() {
            let crosses = lines[i].cross(&lines[j]);
            for cross in crosses {
                let count = cross_counter.entry(cross).or_insert(0);
                *count += 1;
            }
        }
    }

    cross_counter
        .iter()
        .filter(|(_, &count)| count >= 1)
        .count() as i32
}

pub fn solve_part2(input: &str) -> i32 {
    let lines = parse_input(input)
        .iter()
        .filter(|l| l.line_type().part2_type())
        .copied()
        .collect::<Vec<_>>();

    let mut cross_counter: HashMap<(i32, i32), i32> = HashMap::new();
    for i in 0..(lines.len() - 1) {
        for j in (i + 1)..lines.len() {
            let crosses = lines[i].cross(&lines[j]);
            for cross in crosses {
                let count = cross_counter.entry(cross).or_insert(0);
                *count += 1;
            }
        }
    }

    cross_counter
        .iter()
        .filter(|(_, &count)| count >= 1)
        .count() as i32
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Line {
    start: (i32, i32),
    end: (i32, i32),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum LineType {
    Horizontal,
    Vertical,
    Diagonal,
    Point,
    Etc,
}

impl LineType {
    fn part1_type(&self) -> bool {
        match *self {
            LineType::Horizontal | LineType::Vertical | LineType::Point => true,
            _ => false,
        }
    }

    fn part2_type(&self) -> bool {
        match *self {
            LineType::Etc => false,
            _ => true,
        }
    }
}

impl Line {
    fn new(start: (i32, i32), end: (i32, i32)) -> Self {
        Self { start, end }
    }

    fn line_type(&self) -> LineType {
        if self.start == self.end {
            LineType::Point
        } else if self.start.0 == self.end.0 {
            LineType::Vertical
        } else if self.start.1 == self.end.1 {
            LineType::Horizontal
        } else {
            let dx = (self.start.0 - self.end.0).abs();
            let dy = (self.start.1 - self.end.1).abs();
            if dx == dy && dx != 0 {
                LineType::Diagonal
            } else {
                LineType::Etc
            }
        }
    }

    fn min_x(&self) -> i32 {
        i32::min(self.start.0, self.end.0)
    }

    fn max_x(&self) -> i32 {
        i32::max(self.start.0, self.end.0)
    }

    fn min_y(&self) -> i32 {
        i32::min(self.start.1, self.end.1)
    }

    fn max_y(&self) -> i32 {
        i32::max(self.start.1, self.end.1)
    }

    fn has_point(&self, p: (i32, i32)) -> bool {
        match self.line_type() {
            LineType::Point => self.start == p,
            LineType::Horizontal => {
                self.min_x() <= p.0 && p.0 <= self.max_x() && self.start.1 == p.1
            }
            LineType::Vertical => self.min_y() <= p.1 && p.1 <= self.max_y() && self.start.0 == p.0,
            LineType::Diagonal => {
                let a = (self.start.1 - self.end.1) / (self.start.0 - self.end.0);
                let b = self.start.1 - a * self.start.0;
                if p.1 == p.0 * a + b {
                    self.min_x() <= p.0
                        && p.0 <= self.max_x()
                        && self.min_y() <= p.1
                        && p.1 <= self.max_y()
                } else {
                    false
                }
            }
            LineType::Etc => panic!("Etc line type is not supported"),
        }
    }

    fn cross(&self, other: &Line) -> Vec<(i32, i32)> {
        let mut crosses = Vec::new();

        match (self.line_type(), other.line_type()) {
            (LineType::Point, LineType::Point) => {
                if self.has_point(other.start) {
                    crosses.push(self.start);
                }
            }
            (LineType::Point, _) => {
                if other.has_point(self.start) {
                    crosses.push(self.start);
                }
            }
            (_, LineType::Point) => {
                if self.has_point(other.start) {
                    crosses.push(other.start);
                }
            }
            (LineType::Horizontal, LineType::Horizontal) => {
                let p = cross_horizontal_horizontal(self, other);
                crosses.extend(p.into_iter());
            }
            (LineType::Vertical, LineType::Vertical) => {
                let p = cross_vertical_vertical(self, other);
                crosses.extend(p.into_iter());
            }
            (LineType::Horizontal, LineType::Vertical) => {
                if let Some(p) = cross_horizontal_vertical(self, other) {
                    crosses.push(p);
                }
            }
            (LineType::Vertical, LineType::Horizontal) => {
                if let Some(p) = cross_horizontal_vertical(other, self) {
                    crosses.push(p);
                }
            }
            (LineType::Diagonal, LineType::Horizontal) => {
                if let Some(p) = cross_diagonal_horizontal(self, other) {
                    crosses.push(p);
                }
            }
            (LineType::Horizontal, LineType::Diagonal) => {
                if let Some(p) = cross_diagonal_horizontal(other, self) {
                    crosses.push(p);
                }
            }
            (LineType::Diagonal, LineType::Vertical) => {
                if let Some(p) = cross_diagonal_vertical(self, other) {
                    crosses.push(p);
                }
            }
            (LineType::Vertical, LineType::Diagonal) => {
                if let Some(p) = cross_diagonal_vertical(other, self) {
                    crosses.push(p);
                }
            }
            (LineType::Diagonal, LineType::Diagonal) => {
                let p = cross_diagonal_diagonal(self, other);
                crosses.extend(p);
            }
            (LineType::Etc, _) | (_, LineType::Etc) => {
                panic!("Etc line type is not supported");
            }
        }
        crosses
    }
}

fn cross_horizontal_horizontal(horizontal1: &Line, horizontal2: &Line) -> Vec<(i32, i32)> {
    let mut crosses = Vec::new();
    if horizontal1.start.1 == horizontal2.start.1 {
        let start = i32::max(horizontal1.min_x(), horizontal2.min_x());
        let end = i32::min(horizontal1.max_x(), horizontal2.max_x());
        for x in start..=end {
            crosses.push((x, horizontal1.start.1));
        }
    }
    crosses
}

fn cross_vertical_vertical(vertical1: &Line, vertical2: &Line) -> Vec<(i32, i32)> {
    let mut crosses = Vec::new();
    if vertical1.start.0 == vertical2.start.0 {
        let start = i32::max(vertical1.min_y(), vertical2.min_y());
        let end = i32::min(vertical1.max_y(), vertical2.max_y());
        for y in start..=end {
            crosses.push((vertical1.start.0, y));
        }
    }
    crosses
}

fn cross_horizontal_vertical(horizontal: &Line, vertical: &Line) -> Option<(i32, i32)> {
    let x = vertical.start.0;
    if horizontal.min_x() <= x && x <= horizontal.max_x() {
        let y = horizontal.start.1;
        if vertical.min_y() <= y && y <= vertical.max_y() {
            return Some((x, y));
        }
    }
    None
}

fn cross_diagonal_horizontal(diagonal: &Line, horizontal: &Line) -> Option<(i32, i32)> {
    let a = (diagonal.start.1 - diagonal.end.1) / (diagonal.start.0 - diagonal.end.0);
    let b = diagonal.start.1 - a * diagonal.start.0;
    let y = horizontal.start.1;
    let x = (y - b) / a;

    if diagonal.has_point((x, y)) && horizontal.has_point((x, y)) {
        return Some((x, y));
    }
    None
}

fn cross_diagonal_vertical(diagonal: &Line, vertical: &Line) -> Option<(i32, i32)> {
    let a = (diagonal.start.1 - diagonal.end.1) / (diagonal.start.0 - diagonal.end.0);
    let b = diagonal.start.1 - a * diagonal.start.0;
    let x = vertical.start.0;
    let y = a * x + b;

    if diagonal.has_point((x, y)) && vertical.has_point((x, y)) {
        return Some((x, y));
    }
    None
}

fn cross_diagonal_diagonal(left: &Line, right: &Line) -> Vec<(i32, i32)> {
    let a1 = (left.start.0 - left.end.0) / (left.start.1 - left.end.1);
    let b1 = left.start.1 - a1 * left.start.0;
    let a2 = (right.start.0 - right.end.0) / (right.start.1 - right.end.1);
    let b2 = right.start.1 - a2 * right.start.0;

    let mut crosses = Vec::new();

    if a1 * a2 == -1 {
        // cross
        assert!((a1 - a2).abs() % 2 == 0);
        if (b2 - b1).abs() % 2 == 0 {
            let x = (b2 - b1) / (a1 - a2);
            let y = a1 * x + b1;
            if left.has_point((x, y)) && right.has_point((x, y)) {
                crosses.push((x, y));
            }
        }
    } else {
        let start = i32::max(left.min_x(), right.min_x());
        let end = i32::min(left.max_x(), right.max_x());
        for x in start..=end {
            let y = a1 * x + b1;
            if right.has_point((x, y)) {
                crosses.push((x, y));
            }
        }
    }
    crosses
}

fn parse_input(input: &str) -> Vec<Line> {
    let mut lines = Vec::new();
    for line in input.lines() {
        let tokens = line
            .split(" -> ")
            .map(|s| s.split(','))
            .flatten()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        assert_eq!(tokens.len(), 4);
        lines.push(Line::new((tokens[0], tokens[1]), (tokens[2], tokens[3])));
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;

    #[test]
    fn test_parse() {
        let lines = parse_input(SAMPLE_INPUT);
        assert_eq!(lines.len(), 10);
        assert_eq!(lines[0], Line::new((0, 9), (5, 9)));
        assert_eq!(lines[9], Line::new((5, 5), (8, 2)));
    }

    #[test]
    fn test_cross() {
        let line1 = Line::new((7, 0), (7, 4));
        let line2 = Line::new((9, 4), (3, 4));
        let crosses = line1.cross(&line2);
        assert_eq!(crosses, vec![(7, 4)]);

        // cross line
        let line1 = Line::new((2, 2), (4, 2));
        let line2 = Line::new((3, 1), (3, 3));
        let crosses = line1.cross(&line2);
        assert_eq!(crosses, vec![(3, 2)]);

        // cross but not intersect
        let line1 = Line::new((2, 2), (4, 2));
        let line2 = Line::new((3, 3), (3, 4));
        let crosses = line1.cross(&line2);
        assert_eq!(crosses, vec![]);

        // horizontal overalped line
        let line1 = Line::new((2, 2), (4, 2));
        let line2 = Line::new((1, 2), (3, 2));
        let crosses = line1.cross(&line2);
        assert_eq!(crosses, vec![(2, 2), (3, 2)]);

        // horizontal non-overalped line
        let line1 = Line::new((2, 2), (4, 2));
        let line2 = Line::new((5, 2), (9, 2));
        let crosses = line1.cross(&line2);
        assert_eq!(crosses, vec![]);

        // vertical overalped line
        let line1 = Line::new((2, 2), (2, 4));
        let line2 = Line::new((2, 1), (2, 3));
        let crosses = line1.cross(&line2);
        assert_eq!(crosses, vec![(2, 2), (2, 3)]);

        // vertical non-overalped line
        let line1 = Line::new((2, 2), (2, 4));
        let line2 = Line::new((2, 5), (2, 9));
        let crosses = line1.cross(&line2);
        assert_eq!(crosses, vec![]);

        // diagonal line x diagonal line (parallel)
        let line1 = Line::new((2, 2), (4, 4));
        let line2 = Line::new((3, 1), (5, 3));
        let crosses = line1.cross(&line2);
        assert_eq!(crosses, vec![]);

        let line1 = Line::new((2, 2), (4, 4));
        let line2 = Line::new((6, 6), (3, 3));
        let crosses = line1.cross(&line2);
        assert_eq!(crosses, vec![(3, 3), (4, 4)]);

        // diagonal line x diagonal line (cross)
        let line1 = Line::new((2, 2), (4, 4));
        let line2 = Line::new((4, 2), (2, 4));
        let crosses = line1.cross(&line2);
        assert_eq!(crosses, vec![(3, 3)]);

        let line1 = Line::new((2, 2), (5, 5));
        let line2 = Line::new((5, 2), (2, 5));
        let crosses = line1.cross(&line2);
        assert_eq!(crosses, vec![]);

        let line1 = Line::new((0, 0), (2, 2));
        let line2 = Line::new((3, 3), (5, 5));
        let crosses = line1.cross(&line2);
        assert_eq!(crosses, vec![]);

        // diagonal line x horizontal line
        let line1 = Line::new((0, 0), (10, 10));
        let line2 = Line::new((0, 11), (12, 11));
        let crosses = line1.cross(&line2);
        assert_eq!(crosses, vec![]);
    }

    #[test]
    fn test_part1_sample() {
        let answer = solve_part1(SAMPLE_INPUT);
        assert_eq!(answer, 5);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input/day_05.txt");
        let answer = solve_part1(input);
        assert_eq!(answer, 5442);
    }

    #[test]
    fn test_part2_sample() {
        let answer = solve_part2(SAMPLE_INPUT);
        assert_eq!(answer, 12);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input/day_05.txt");
        let answer = solve_part2(input);
        assert_eq!(answer, 19571);
    }
}
