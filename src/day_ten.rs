use std::collections::HashMap;

pub fn check_balanced(s: &str) -> usize {
    let mut stack: Vec<char> = Vec::new();
    let matching_closes: HashMap<char, (char, usize)> = HashMap::from([
        (')', ('(', 3)),
        (']', ('[', 57)),
        ('}', ('{', 1197)),
        ('>', ('<', 25137)),
    ]);

    for c in s.chars() {
        if matching_closes.contains_key(&c) {
            let (matching, value) = matching_closes.get(&c).unwrap();
            if stack.is_empty() {
                return *value;
            } else {
                let last_match = stack.pop().unwrap();
                if last_match != *matching {
                    return *value;
                }
            }
        } else {
            stack.push(c);
        }
    }
    0
}

#[cfg(test)]
mod test {
    use super::check_balanced;

    #[test]
    fn test_check_balanced() {
        assert_eq!(1197, check_balanced("{([(<{}[<>[]}>{[]{[(<()>"));
        assert_eq!(3, check_balanced("[[<[([]))<([[{}[[()]]]"));
        assert_eq!(57, check_balanced("[{[{({}]{}}([{[{{{}}([]"));
        assert_eq!(3, check_balanced("[<(<(<(<{}))><([]([]()"));
        assert_eq!(25137, check_balanced("<{([([[(<>()){}]>(<<{{"));
        assert_eq!(0, check_balanced("[({(<(())[]>[[{[]{<()<>>"));
    }
}