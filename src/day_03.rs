pub fn solve_part1(input: &str) -> u32 {
    let first = input.lines().next().unwrap().trim();
    let count = first.len();
    let mut common = Vec::new();
    common.resize(count, 0);

    for line in input.lines() {
        let line = line.trim();
        for (idx, c) in line.char_indices() {
            println!("{} {}", idx, c);
            if c == '0' {
                common[idx] -= 1;
            } else {
                common[idx] += 1;
            }
        }
    }

    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for c in &common {
        gamma_rate <<= 1;
        gamma_rate |= if *c > 0 { 1 } else { 0 };

        epsilon_rate <<= 1;
        epsilon_rate |= if *c < 0 { 1 } else { 0 };
    }

    gamma_rate * epsilon_rate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let input = r#"00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010"#;
        let answer = solve_part1(input);
        assert_eq!(answer, 198);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input/day_03.txt");
        let answer = solve_part1(input);
        assert_eq!(answer, 852500);
    }
}
