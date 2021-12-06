use std::collections::HashMap;

const NEW_COUNTER: i64 = 8;
const INITIAL_COUNTER: i64 = 6;

pub fn calculate_lanternfish(lanternfish: Vec<i64>, days: i64) -> i64 {
    // key of counters is <day, counter>, value is num fish produced
    let mut counters: HashMap<(i64, i64), i64> = HashMap::new();
    for day in 0..=days {
        for timer in (0..=NEW_COUNTER).rev() {
            if day == 0 {
                counters.insert((day, timer), 1);
            } else {
                let next_timer: i64;
                let offset: i64;
                if timer == 0 {
                    next_timer = INITIAL_COUNTER;
                    offset = *counters.get(&(day - 1, NEW_COUNTER)).unwrap();
                } else {
                    next_timer = timer - 1;
                    offset = 0;
                }
                let num_fish = *counters.get(&(day - 1, next_timer)).unwrap();
                counters.insert((day, timer), num_fish + offset);
            }
        }
    }

    lanternfish
        .iter()
        .map(|&x| counters.get(&(days, x)).unwrap())
        .sum()
}

#[cfg(test)]
mod test {
    use super::calculate_lanternfish;

    #[test]
    fn test_calculate_lanternfish() {
        assert_eq!(26, calculate_lanternfish(vec![3, 4, 3, 1, 2], 18));
        assert_eq!(5934, calculate_lanternfish(vec![3, 4, 3, 1, 2], 80));
        assert_eq!(26984457539, calculate_lanternfish(vec![3, 4, 3, 1, 2], 256))
    }
}
