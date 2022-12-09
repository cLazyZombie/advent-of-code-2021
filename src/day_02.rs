use std::str::FromStr;

#[derive(Debug)]
enum Command {
    Forward,
    Up,
    Down,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Command::Forward),
            "up" => Ok(Command::Up),
            "down" => Ok(Command::Down),
            _ => Err(()),
        }
    }
}

pub fn solve_part1(input: &str) -> u32 {
    let mut x = 0;
    let mut depth = 0;
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let command = parts.next().unwrap().parse::<Command>().unwrap();
        let value = parts.next().unwrap().parse::<u32>().unwrap();

        match command {
            Command::Forward => x += value,
            Command::Up => depth -= value,
            Command::Down => depth += value,
        }
    }

    x * depth
}

pub fn solve_part2(input: &str) -> u32 {
    let mut aim = 0;
    let (mut horizon, mut depth) = (0, 0);
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let command = parts.next().unwrap().parse::<Command>().unwrap();
        let value = parts.next().unwrap().parse::<u32>().unwrap();

        match command {
            Command::Forward => {
                horizon += value;
                depth += aim * value;
            }
            Command::Up => aim -= value,
            Command::Down => aim += value,
        }
    }

    horizon * depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let input = r#"forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2"#;
        let answer = solve_part1(input);
        assert_eq!(answer, 150);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input/day_02.txt");
        let answer = solve_part1(input);
        assert_eq!(answer, 1698735);
    }

    #[test]
    fn test_part2_sample() {
        let input = r#"forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2"#;
        let answer = solve_part2(input);
        assert_eq!(answer, 900);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input/day_02.txt");
        let answer = solve_part2(input);
        assert_eq!(answer, 1594785890);
    }
}
