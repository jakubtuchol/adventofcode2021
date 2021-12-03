pub fn get_gamma_epsilon_product(bin_nums: Vec<String>) -> i32 {
    if bin_nums.len() == 0 {
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
    
    return gamma * epsilon;
}

#[cfg(test)]
mod tests {
    use super::get_gamma_epsilon_product;

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
}