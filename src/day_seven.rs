use std::collections::HashMap;

pub fn constant_gas(pos: Vec<i32>) -> i32 {
    let mut seen_pos: HashMap<i32, i32> = HashMap::new();

    let min_pos = match pos.iter().min() {
        Some(&x) => x,
        None => panic!("no min for vector"),
    };
    let max_pos = match pos.iter().max() {
        Some(&x) => x,
        None => panic!("no max for vector"),
    };

    for target in min_pos..=max_pos {
        if seen_pos.contains_key(&target) {
            continue;
        }
        seen_pos.insert(target, pos.iter().map(|&x| (target - x).abs()).sum());
    }
    match seen_pos.values().min() {
        Some(&m) => m,
        None => panic!("no values inserted into map"),
    }
}

pub fn variable_gas(pos: Vec<i32>) -> i32 {
    let mut seen_pos: HashMap<i32, i32> = HashMap::new();

    let min_pos = match pos.iter().min() {
        Some(&x) => x,
        None => panic!("no min for vector"),
    };
    let max_pos = match pos.iter().max() {
        Some(&x) => x,
        None => panic!("no max for vector"),
    };

    for target in min_pos..=max_pos {
        if seen_pos.contains_key(&target) {
            continue;
        }
        seen_pos.insert(
            target,
            pos.iter()
                .map(|&x| {
                    let distance: i32 = (target - x).abs();
                    (distance * (distance + 1)) / 2
                })
                .sum(),
        );
    }
    match seen_pos.values().min() {
        Some(&m) => m,
        None => panic!("no values inserted into map"),
    }
}

#[cfg(test)]
mod test {
    use super::{constant_gas, variable_gas};

    #[test]
    fn test_constant_gas() {
        assert_eq!(37, constant_gas(vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]),);
    }

    #[test]
    fn test_variable_gas() {
        assert_eq!(168, variable_gas(vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]),);
    }
}
