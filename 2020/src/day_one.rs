use std::collections::HashSet;

pub fn two_sum(v: Vec<i32>, target: i32) -> Option<i32> {
    let mut seen: HashSet<i32> = HashSet::new();
    for &elt in v.iter() {
        let remainder = target - elt;
        if seen.contains(&elt) {
            return Some(remainder * elt);
        }
        seen.insert(remainder);
    }
    None
}

pub fn three_sum(v: Vec<i32>, target: i32) -> Option<i32> {
    for (idx, &elt) in v.iter().enumerate() {
        let remaining_nums: Vec<i32> = v
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != idx)
            .map(|(_, &e)| e)
            .collect();
        let remainder = two_sum(remaining_nums, target - elt);
        if let Some(r) = remainder {
            return Some(elt * r);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::{three_sum, two_sum};

    #[test]
    fn test_two_sum() {
        let v = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(514579, two_sum(v, 2020).unwrap());
    }

    #[test]
    fn test_three_sum() {
        let v = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(241861950, three_sum(v, 2020).unwrap());
    }
}
