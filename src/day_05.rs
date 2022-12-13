use std::collections::HashMap;

pub fn solve_part1(input: &str) -> i32 {
    let lines = parse_input(input)
        .iter()
        .filter(|l| l.is_ortho())
        .copied()
        .collect::<Vec<_>>();

    let mut cross_counter: HashMap<(i32, i32), i32> = HashMap::new();
    for i in 0..(lines.len() - 1) {
        for j in (i + 1)..lines.len() {
            let crosses = lines[i].cross(&lines[j]);
            println!("crosses: {:?}", crosses);
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

impl Line {
    fn new(start: (i32, i32), end: (i32, i32)) -> Self {
        Self { start, end }
    }

    fn is_ortho(&self) -> bool {
        self.start.0 == self.end.0 || self.start.1 == self.end.1
    }

    // fn is_diagonal(&self) -> bool {
    //     (self.start.0 - self.end.0).abs() == (self.start.1 - self.end.1).abs()
    // }

    fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }

    fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
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

    fn cross(&self, other: &Line) -> Vec<(i32, i32)> {
        assert!(self.is_ortho() && other.is_ortho());
        let mut crosses = Vec::new();

        if (self.is_horizontal() && other.is_vertical())
            || (self.is_vertical() && other.is_horizontal())
        {
            let (horizontal, vertical) = if self.is_horizontal() {
                (*self, *other)
            } else {
                (*other, *self)
            };

            let x = vertical.start.0;
            if horizontal.min_x() <= x && x <= horizontal.max_x() {
                let y = horizontal.start.1;
                if vertical.min_y() <= y && y <= vertical.max_y() {
                    crosses.push((x, y));
                }
            }
        } else if self.is_horizontal() && other.is_horizontal() {
            if self.start.1 == other.start.1 {
                let start = i32::max(self.min_x(), other.min_x());
                let end = i32::min(self.max_x(), other.max_x());
                for x in start..=end {
                    crosses.push((x, self.start.1));
                }
            }
        } else if self.is_vertical() && other.is_vertical() {
            if self.start.0 == other.start.0 {
                let start = i32::max(self.min_y(), other.min_y());
                let end = i32::min(self.max_y(), other.max_y());
                for y in start..=end {
                    crosses.push((self.start.0, y));
                }
            }
        }
        crosses
    }
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
}
