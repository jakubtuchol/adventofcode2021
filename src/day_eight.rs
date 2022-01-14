use std::collections::HashSet;

pub fn get_unique_digits(input: Vec<String>) -> usize {
    let mut appearances = 0;
    // 1 -> 2 segments, 4 -> 4 segments, 7 -> 3 segments, 8 -> 7 segments
    let unique_sizes: HashSet<usize> = vec![2,4,3,7].into_iter().collect();

    for s in input {
        let outputs = s.split("|").nth(1).unwrap();
        appearances += outputs.trim().split_ascii_whitespace().filter(|&num| unique_sizes.contains(&num.len())).count();
    }
    appearances
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