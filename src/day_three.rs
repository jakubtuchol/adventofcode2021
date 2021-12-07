pub fn get_gamma_epsilon_product(bin_nums: Vec<String>) -> i32 {
    if bin_nums.is_empty() {
        return 0;
    }

    let num_len = bin_nums[0].len();
    let mut counters: Vec<i32> = vec![0; num_len];

    for num in bin_nums.iter() {
        for (idx, c) in num.chars().enumerate() {
            if c == '0' {
                counters[idx] -= 1;
            } else if c == '1' {
                counters[idx] += 1;
            }
        }
    }

    let mut base = 1;
    let mut gamma = 0;
    let mut epsilon = 0;

    for num in counters.iter().rev() {
        if *num > 0 {
            gamma += base;
        } else {
            epsilon += base;
        }
        base *= 2;
    }
    gamma * epsilon
}

pub fn get_oxygen_co2_product(nums: Vec<String>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut most_common_filtered = nums.clone();
    let mut least_common_filtered = nums.clone();

    for idx in 0..nums[0].len() {
        let mut most_common = 0;
        let mut least_common = 0;

        for num in most_common_filtered.iter() {
            if num.chars().nth(idx).unwrap() == '1' {
                most_common += 1;
            } else {
                most_common -= 1;
            }
        }

        for num in least_common_filtered.iter() {
            if num.chars().nth(idx).unwrap() == '1' {
                least_common += 1;
            } else {
                least_common -= 1;
            }
        }

        let most_common_char: char;
        if most_common >= 0 {
            most_common_char = '1';
        } else {
            most_common_char = '0';
        }

        let least_common_char: char;
        if least_common >= 0 {
            least_common_char = '0';
        } else {
            least_common_char = '1';
        }

        if most_common_filtered.len() > 1 {
            most_common_filtered = most_common_filtered
                .into_iter()
                .filter(|x| x.chars().nth(idx).unwrap() == most_common_char)
                .collect();
        }
        if least_common_filtered.len() > 1 {
            least_common_filtered = least_common_filtered
                .into_iter()
                .filter(|x| x.chars().nth(idx).unwrap() == least_common_char)
                .collect();
        }
    }

    let mut oxygen_rating = 0;
    let mut carbon_rating = 0;
    let mut base = 1;

    if most_common_filtered.len() != 1 {
        println!("no most common");
        return 0;
    }

    if least_common_filtered.len() != 1 {
        println!("no least common");
        return 0;
    }

    let remaining_ox = most_common_filtered[0].clone();
    let remaining_co = least_common_filtered[0].clone();

    for it in remaining_ox.chars().rev().zip(remaining_co.chars().rev()) {
        let (ox, co2) = it;
        if ox == '1' {
            oxygen_rating += base;
        }
        if co2 == '1' {
            carbon_rating += base;
        }

        base *= 2;
    }

    oxygen_rating * carbon_rating
}

#[cfg(test)]
mod tests {
    use super::{get_gamma_epsilon_product, get_oxygen_co2_product};

    #[test]
    fn test_gamma_epsilon() {
        let nums: Vec<String> = vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ];

        assert_eq!(198, get_gamma_epsilon_product(nums));
    }

    #[test]
    fn test_oxygen_co2() {
        let nums: Vec<String> = vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ];

        assert_eq!(230, get_oxygen_co2_product(nums));
    }
}
