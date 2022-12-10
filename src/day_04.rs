pub fn solve_part1(input: &str) -> u32 {
    let (numbers, mut grids) = parse_input(input);

    for n in numbers {
        for grid in grids.iter_mut() {
            mark(n, grid);
            if is_winner(grid) {
                let sum: u32 = sum_unmarked(grid);
                return sum * n;
            }
        }
    }

    0
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Number {
    Marked(u32),
    Unmarked(u32),
}

impl Number {
    fn is_marked(&self) -> bool {
        match self {
            Number::Marked(_) => true,
            Number::Unmarked(_) => false,
        }
    }

    fn value(&self) -> u32 {
        match self {
            Number::Marked(n) => *n,
            Number::Unmarked(n) => *n,
        }
    }
}

fn sum_unmarked(grid: &[[Number; 5]; 5]) -> u32 {
    grid.iter()
        .flatten()
        .filter(|m| !m.is_marked())
        .map(|m| m.value())
        .sum()
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<[[Number; 5]; 5]>) {
    let mut lines = input.lines();
    let numbers = lines
        .next()
        .unwrap()
        .trim()
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut grids = Vec::new();

    while let Some(_) = lines.next() {
        let mut grid = [[Number::Unmarked(0); 5]; 5];

        for y in 0..5 {
            let line = lines.next().unwrap();
            for (x, number) in line.trim().split_ascii_whitespace().enumerate() {
                grid[y][x] = Number::Unmarked(number.parse::<u32>().unwrap());
            }
        }

        grids.push(grid);
    }

    (numbers, grids)
}

fn mark(num: u32, grid: &mut [[Number; 5]; 5]) {
    for y in 0..5 {
        for x in 0..5 {
            if let Number::Unmarked(n) = grid[y][x] {
                if n == num {
                    grid[y][x] = Number::Marked(num);
                    return;
                }
            }
        }
    }
}

fn is_winner(grid: &[[Number; 5]; 5]) -> bool {
    // horizontal
    for y in 0..5 {
        if (0..5).all(|x| grid[y][x].is_marked()) {
            return true;
        }
    }

    // vertical
    for x in 0..5 {
        if (0..5).all(|y| grid[y][x].is_marked()) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let input = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19
        
         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6
        
        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7"#;
        let answer = solve_part1(input);
        assert_eq!(answer, 4512);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input/day_04.txt");
        let answer = solve_part1(input);
        assert_eq!(answer, 39984);
    }
}
