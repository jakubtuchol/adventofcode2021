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

pub fn find_completion(s: &str) -> usize {
    let mut completion: Vec<char> = Vec::new();
    let mut stack: Vec<char> = Vec::new();
    let close_matchings = HashMap::from([
        (')', '('),
        (']', '['),
        ('}', '{'),
        ('>', '<'),
    ]);
    let open_matchings = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ]);

    for c in s.chars() {
        if close_matchings.contains_key(&c) {
            if stack.is_empty() {
                return 0;
            }
            let open = close_matchings.get(&c).unwrap();
            let mut last = stack.pop().unwrap();
        }
        /*
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
 */
    }
    0

}

pub fn get_completion_score(s: &str) -> usize {
    let points = HashMap::from([
        (')', 1),
        (']', 2),
        ('}', 3),
        ('>', 4),
    ]);

    s.chars().fold(0, |acc, x| {
        let value = points.get(&x).unwrap();
        acc * 5 + value
    })
}

#[cfg(test)]
mod test {
    use super::{check_balanced, get_completion_score};

    #[test]
    fn test_check_balanced() {
        assert_eq!(1197, check_balanced("{([(<{}[<>[]}>{[]{[(<()>"));
        assert_eq!(3, check_balanced("[[<[([]))<([[{}[[()]]]"));
        assert_eq!(57, check_balanced("[{[{({}]{}}([{[{{{}}([]"));
        assert_eq!(3, check_balanced("[<(<(<(<{}))><([]([]()"));
        assert_eq!(25137, check_balanced("<{([([[(<>()){}]>(<<{{"));
        assert_eq!(0, check_balanced("[({(<(())[]>[[{[]{<()<>>"));
    }

    #[test]
    fn test_completion_score() {
        assert_eq!(294, get_completion_score("])}>"));
        assert_eq!(288957, get_completion_score("}}]])})]"));
        assert_eq!(5566, get_completion_score(")}>]})"));
        assert_eq!(1480781, get_completion_score("}}>}>))))"));
        assert_eq!(995444, get_completion_score("]]}}]}]}>"));
    }
}