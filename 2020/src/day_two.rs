pub struct PasswordPolicy {
    letter: char,
    min: usize,
    max: usize,
    password: String,
}

impl Clone for PasswordPolicy {
    fn clone(&self) -> PasswordPolicy {
        PasswordPolicy::new(self.letter, self.min, self.max, self.password.clone())
    }
}

impl PasswordPolicy {
    pub fn new(letter: char, min: usize, max: usize, password: String) -> PasswordPolicy {
        Self {
            letter,
            min,
            max,
            password,
        }
    }

    pub fn valid_password(&self) -> bool {
        let num_matching = self.password.chars().filter(|&c| c == self.letter).count();
        num_matching >= self.min && num_matching <= self.max
    }

    pub fn valid_position_password(&self) -> bool {
        let mut min_match = false;
        let mut max_match = false;

        for (idx, elt) in self.password.chars().enumerate() {
            if idx + 1 == self.min {
                min_match = self.letter == elt;
            }
            if idx + 1 == self.max {
                max_match = self.letter == elt;
            }
        }
        vec![min_match, max_match].iter().filter(|&x| *x).count() == 1
    }
}

pub fn count_valid_passwords(policies: Vec<PasswordPolicy>) -> usize {
    policies
        .iter()
        .filter(|policy| policy.valid_password())
        .count()
}

pub fn count_valid_position_passwords(policies: Vec<PasswordPolicy>) -> usize {
    policies
        .iter()
        .filter(|policy| policy.valid_position_password())
        .count()
}

#[cfg(test)]
mod test {
    use super::PasswordPolicy;

    #[test]
    fn test_valid_password() {
        let first_policy = PasswordPolicy::new('a', 1, 3, "abcde".to_owned());
        let second_policy = PasswordPolicy::new('b', 1, 3, "cdefg".to_owned());
        let third_policy = PasswordPolicy::new('c', 2, 9, "ccccccccc".to_owned());

        assert_eq!(true, first_policy.valid_password());
        assert_eq!(false, second_policy.valid_password());
        assert_eq!(true, third_policy.valid_password());
    }

    #[test]
    fn test_valid_position_password() {
        let first_policy = PasswordPolicy::new('a', 1, 3, "abcde".to_owned());
        let second_policy = PasswordPolicy::new('b', 1, 3, "cdefg".to_owned());
        let third_policy = PasswordPolicy::new('c', 2, 9, "ccccccccc".to_owned());

        assert_eq!(true, first_policy.valid_position_password());
        assert_eq!(false, second_policy.valid_position_password());
        assert_eq!(false, third_policy.valid_position_password());
    }
}
