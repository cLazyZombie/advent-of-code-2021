pub fn solve_part1(input: &str) -> u32 {
    let count = bit_count(input);
    let input_bits = input_to_bits(input);
    let gamma_rate = most_common_bits(count, &input_bits);
    let epsilon_rate = least_common_bits(count, &input_bits);

    gamma_rate * epsilon_rate
}

pub fn solve_part2(input: &str) -> u32 {
    let count = bit_count(input);

    // oxygen
    let mut remained = input_to_bits(input);
    let mut oxygen = 0;
    for bit_idx in (0..count).rev() {
        let most_bits = most_common_bits(count, &remained);
        remained.retain(|&bit| (bit & 1 << bit_idx) == (most_bits & 1 << bit_idx));
        if remained.len() == 1 {
            oxygen = remained[0];
            break;
        }
    }

    // co2
    let mut remained = input_to_bits(input);
    let mut co2 = 0;
    for idx in (0..count).rev() {
        let least_bits = least_common_bits(count, &remained);
        remained.retain(|&bit| bit & 1 << idx == least_bits & 1 << idx);
        if remained.len() == 1 {
            co2 = remained[0];
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
        result |= u32::from(c == '1');
    }
    result
}

fn most_common_bits(len: u32, bits: &Vec<u32>) -> u32 {
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

fn least_common_bits(len: u32, bits: &Vec<u32>) -> u32 {
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
