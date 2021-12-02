use std::collections::VecDeque;

pub fn get_num_increases(depths: Vec<i32>) -> i32 {
    let mut last_depth: i32 = 0;
    let mut increases: i32 = 0;

    for (idx, elt) in depths.iter().enumerate() {
        if idx != 0 {
            if *elt > last_depth {
                increases += 1;
            }
        }

        last_depth = *elt;
    }

    return increases;
}

pub fn get_sliding_window_increases(depths: Vec<i32>) -> i32 {
    let mut last_sum: i32 = 0;
    let mut window: VecDeque<i32> = VecDeque::new();
    let mut increases: i32 = 0;

    for elt in depths.iter() {
        if window.len() == 3 {
            let remove = window.pop_front().unwrap();
            let new_sum = last_sum - remove + elt;

            if new_sum > last_sum {
                increases += 1;
            }
            last_sum = new_sum;
        }
        window.push_back(*elt);
    }

    return increases;
}

#[cfg(test)]
mod tests {
    use super::{get_num_increases, get_sliding_window_increases};

    #[test]
    fn test_num_increases() {
        let depths = vec![
            199, 200, 208, 210, 200, 207, 240, 269, 260, 263,
        ];

        assert_eq!(7, get_num_increases(depths));
    }

    #[test]
    fn test_sliding_window_increases() {
        let depths = vec![
            199, 200, 208, 210, 200, 207, 240, 269, 260, 263,
        ];

        assert_eq!(5, get_sliding_window_increases(depths));
    }
}