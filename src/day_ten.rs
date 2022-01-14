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
    println!("LOOKING AT: {}", s);
    let mut stack: Vec<char> = Vec::new();
    let close_matchings = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    for c in s.chars() {
        if close_matchings.contains_key(&c) {
            let &open = close_matchings.get(&c).unwrap();
            println!("GOT STACK: {:?}, OPEN: {}", stack, c);
            loop {
                //let last = stack.pop().unwrap();
                if *stack.last().unwrap() == open {
                    stack.pop();
                } else {
                    break;
                }
            }
        } else {
            stack.push(c);
        }
    }
    let incomplete: String = stack.iter().rev().collect();
    get_completion_score(&incomplete[..])
}

pub fn get_completion_score(s: &str) -> usize {
    let points = HashMap::from([
        (')', 1),
        (']', 2),
        ('}', 3),
        ('>', 4),
        ('(', 1),
        ('[', 2),
        ('{', 3),
        ('<', 4),
    ]);

    s.chars().fold(0, |acc, x| {
        let value = points.get(&x).unwrap();
        acc * 5 + value
    })
}

pub fn get_middle_completion_score(incomplete: Vec<String>) -> usize {
    let mut scores: Vec<usize> = incomplete
        .iter()
        .map(|x| find_completion(&x.clone()[..]))
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod test {
    use super::{
        check_balanced, find_completion, get_completion_score, get_middle_completion_score,
    };

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

    #[test]
    fn test_find_completion() {
        assert_eq!(288957, find_completion("[({(<(())[]>[[{[]{<()<>>"));
        assert_eq!(5566, find_completion("[(()[<>])]({[<{<<[]>>("));
        assert_eq!(1480781, find_completion("(((({<>}<{<{<>}{[]{[]{}"));
        assert_eq!(995444, find_completion("{<[[]]>}<{[{[{[]{()[[[]"));
        assert_eq!(294, find_completion("<{([{{}}[<[[[<>{}]]]>[]]"));
    }

    #[test]
    fn test_get_middle_completion_score() {
        let v: Vec<String> = vec![
            "[({(<(())[]>[[{[]{<()<>>".to_string(),
            "[(()[<>])]({[<{<<[]>>(".to_string(),
            "(((({<>}<{<{<>}{[]{[]{}".to_string(),
            "{<[[]]>}<{[{[{[]{()[[[]".to_string(),
            "<{([{{}}[<[[[<>{}]]]>[]]".to_string(),
        ];

        assert_eq!(288957, get_middle_completion_score(v));
    }
}
