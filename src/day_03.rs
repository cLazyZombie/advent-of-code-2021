pub fn solve_part1(input: &str) -> u32 {
    let bits = count_bits(input);

    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for c in &bits {
        gamma_rate <<= 1;
        gamma_rate |= if *c > 0 { 1 } else { 0 };

        epsilon_rate <<= 1;
        epsilon_rate |= if *c < 0 { 1 } else { 0 };
    }

    gamma_rate * epsilon_rate
}

pub fn solve_part2(input: &str) -> u32 {
    let count = bit_count(input);

    // oxygen
    let mut bits = input_to_bits(input);
    let mut oxygen = 0;
    for bit_idx in (0..count).rev() {
        let common = most_common(count, &bits);
        bits = bits
            .iter()
            .filter(|&bit| (bit & 1 << bit_idx) == (common & 1 << bit_idx))
            .copied()
            .collect();
        if bits.len() == 1 {
            oxygen = bits[0];
            break;
        }
    }

    // co2
    let mut bits = input_to_bits(input);
    let mut co2 = 0;
    for idx in (0..count).rev() {
        let least = least_common(count, &bits);
        bits = bits
            .iter()
            .filter(|&bit| bit & 1 << idx == least & 1 << idx)
            .copied()
            .collect();

        if bits.len() == 1 {
            co2 = bits[0];
            break;
        }
    }

    println!("oxygen: {:b}", oxygen);
    println!("co2: {:b}", co2);

    oxygen * co2
}

fn input_to_bits(input: &str) -> Vec<u32> {
    input.lines().map(|line| to_bits(line.trim())).collect()
}

fn bit_count(input: &str) -> u32 {
    let first = input.lines().next().unwrap().trim();
    first.len() as u32
}

fn to_bits(input: &str) -> u32 {
    let mut result = 0;
    for c in input.chars() {
        result <<= 1;
        result |= if c == '1' { 1 } else { 0 };
    }
    result
}

fn count_bits(input: &str) -> Vec<i32> {
    let first = input.lines().next().unwrap().trim();
    let count = first.len();
    let mut common = Vec::new();
    common.resize(count, 0);

    for line in input.lines() {
        let line = line.trim();
        for (idx, c) in line.char_indices() {
            if c == '0' {
                common[idx] -= 1;
            } else {
                common[idx] += 1;
            }
        }
    }
    common
}

fn most_common(len: u32, bits: &Vec<u32>) -> u32 {
    let mut result = 0;
    for bit_idx in 0..len {
        let mut count_1 = 0;
        for b in bits {
            if b & (1 << bit_idx) != 0 {
                count_1 += 1;
            } else {
                count_1 -= 1;
            }
        }

        if count_1 >= 0 {
            result |= 1 << bit_idx;
        }
    }

    result
}

fn least_common(len: u32, bits: &Vec<u32>) -> u32 {
    let mut result = 0;
    for bit_idx in 0..len {
        let mut count_1 = 0;
        for b in bits {
            if b & (1 << bit_idx) != 0 {
                count_1 += 1;
            } else {
                count_1 -= 1;
            }
        }

        if count_1 < 0 {
            result |= 1 << bit_idx;
        }
    }

    result
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

    #[test]
    fn test_part2_sample() {
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
        let answer = solve_part2(input);
        assert_eq!(answer, 230);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input/day_03.txt");
        let answer = solve_part2(input);
        assert_eq!(answer, 1007985);
    }
}
