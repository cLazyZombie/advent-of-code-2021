pub fn solve_part1(input: &str) -> u32 {
    let mut prev = None;
    let mut increased = 0;
    for line in input.lines() {
        let num = line.trim().parse::<u32>().unwrap();
        if let Some(prev) = prev {
            if num > prev {
                increased += 1;
            }
        }

        prev = Some(num);
    }
    increased
}

pub fn solve_part2(input: &str) -> u32 {
    let mut prev = None;
    let mut cur = [0; 3];
    let mut increased = 0;
    for (idx, line) in input.lines().enumerate() {
        let num = line.trim().parse::<u32>().unwrap();
        cur[idx % 3] = num;

        if idx >= 2 {
            let sum: u32 = cur.iter().sum();
            if let Some(prev) = prev {
                if sum > prev {
                    increased += 1;
                }
            }

            prev = Some(sum);
        }
    }
    increased
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let input = r#"199
        200
        208
        210
        200
        207
        240
        269
        260
        263"#;
        let answer = solve_part1(input);
        assert_eq!(answer, 7);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input/day_01.txt");
        let answer = solve_part1(input);
        assert_eq!(answer, 1215);
    }

    #[test]
    fn test_part2_sample() {
        let input = r#"199
        200
        208
        210
        200
        207
        240
        269
        260
        263"#;
        let answer = solve_part2(input);
        assert_eq!(answer, 5);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input/day_01.txt");
        let answer = solve_part2(input);
        assert_eq!(answer, 1150);
    }
}
