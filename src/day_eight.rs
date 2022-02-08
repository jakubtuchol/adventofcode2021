use std::collections::{HashSet, HashMap};

pub fn get_unique_digits(input: Vec<String>) -> usize {
    let mut appearances = 0;
    // 1 -> 2 segments, 4 -> 4 segments, 7 -> 3 segments, 8 -> 7 segments
    let unique_sizes: HashSet<usize> = vec![2, 4, 3, 7].into_iter().collect();

    for s in input {
        let outputs = s.split("|").nth(1).unwrap();
        appearances += outputs
            .trim()
            .split_ascii_whitespace()
            .filter(|&num| unique_sizes.contains(&num.len()))
            .count();
    }
    appearances
}

pub fn determine_output_value(input: String) -> usize {
    let digits_to_segments: Vec<HashSet<char>> = vec![
        /*
        (0, vec!['a','b','c','e','f','g'].into_iter().collect::<HashSet<char>>()),
        (1, vec!['c','f'].into_iter().collect::<HashSet<char>>()),
        (2, vec!['a','c','d','e','g'].into_iter().collect::<HashSet<char>>()),
        (3, vec!['a','c','d','f','g'].into_iter().collect::<HashSet<char>>()),
        (4, vec!['b','c','d','f'].into_iter().collect::<HashSet<char>>()),
        (5, vec!['a','b','d','f','g'].into_iter().collect::<HashSet<char>>()),
        (6, vec!['a','b','d','e','f','g'].into_iter().collect::<HashSet<char>>()),
        (7, vec!['a','c','f'].into_iter().collect::<HashSet<char>>()),
        (8, vec!['a','b','c','d','e','f','g'].into_iter().collect::<HashSet<char>>()),
        (9, vec!['a','b','c','d','f','g'].into_iter().collect::<HashSet<char>>()),
        */
        vec!['a','b','c','e','f','g'].into_iter().collect::<HashSet<char>>(),
        vec!['c','f'].into_iter().collect::<HashSet<char>>(),
        vec!['a','c','d','e','g'].into_iter().collect::<HashSet<char>>(),
        vec!['a','c','d','f','g'].into_iter().collect::<HashSet<char>>(),
        vec!['b','c','d','f'].into_iter().collect::<HashSet<char>>(),
        vec!['a','b','d','f','g'].into_iter().collect::<HashSet<char>>(),
        vec!['a','b','d','e','f','g'].into_iter().collect::<HashSet<char>>(),
        vec!['a','c','f'].into_iter().collect::<HashSet<char>>(),
        vec!['a','b','c','d','e','f','g'].into_iter().collect::<HashSet<char>>(),
        vec!['a','b','c','d','f','g'].into_iter().collect::<HashSet<char>>(),
    ];
    let mut possible_mappings: HashMap<char,HashSet<char>> = HashMap::new();
    let mut definite_mappings: HashMap<char, char> = HashMap::new();

    // determine possible mappings
    let segments: Vec<&str> = input.split('|').collect();
    if segments.len() != 2 {
        return 0;
    }

    let all_digits: Vec<&str> = segments[0].split_ascii_whitespace().collect();
    let output_digits: Vec<&str> = segments[1].split_ascii_whitespace().collect();
    
    for &digit in all_digits.iter() {
        let possible: Vec<HashSet<char>> = digits_to_segments.iter().filter(|&set| digit.len() == set.len()).map(|set| set.clone()).collect();
        for c in digit.chars() {
            if definite_mappings.contains_key(&c) {
                continue;
            }
            for &possible_item in possible.iter() {
                if !possible_mappings.contains_key(&c) {
                    possible_mappings.insert(c, possible_item);
                }
            let current_guesses: &HashSet<char> = possible_mappings.get(&c).unwrap_or(&HashSet::new());
            }
        }
    }
    0
}

#[cfg(test)]
mod test {
    use super::get_unique_digits;

    #[test]
    fn test_unique_digits() {
        let inputs = vec![
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe".to_string(),
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc".to_string(),
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg".to_string(),
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb".to_string(),
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea".to_string(),
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb".to_string(),
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe".to_string(),
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef".to_string(),
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb".to_string(),
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce".to_string(),
        ];
        assert_eq!(26, get_unique_digits(inputs));
    }
}
