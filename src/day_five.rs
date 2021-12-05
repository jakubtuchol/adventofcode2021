use std::collections::HashSet;

pub fn get_line(start: (i32, i32), end: (i32, i32)) -> Vec<(i32, i32)> {
    let (start_x, start_y) = start;
    let (end_x, end_y) = end;

    let x_size = match usize::try_from((start_x - end_x).abs()) {
        Ok(size) => size,
        Err(_) => panic!("cannot convert {} into usize", (start_x - end_x).abs()),
    };
    let y_size = match usize::try_from((start_y - end_y).abs()) {
        Ok(size) => size,
        Err(_) => panic!("cannot convert {} into usize", (start_y - end_y).abs()),
    };

    if x_size == 0 && y_size == 0 {
        return vec![start];
    }

    let x_range = if x_size == 0 {
        vec![start_x; y_size + 1]
    } else {
        if start_x > end_x {
            (end_x..=start_x).rev().collect()
        } else {
            (start_x..=end_x).collect()
        }
    };

    let y_range = if y_size == 0 {
        vec![start_y; x_size + 1]
    } else {
        if start_y > end_y {
            (end_y..=start_y).rev().collect()
        } else {
            (start_y..=end_y).collect()
        }
    };

    x_range
        .iter()
        .zip(y_range.iter())
        .map(|(&x, &y)| (x, y))
        .collect::<Vec<(i32, i32)>>()
}

pub fn is_straight(start: (i32, i32), end: (i32, i32)) -> bool {
    let (start_x, start_y) = start;
    let (end_x, end_y) = end;

    if start_x == end_x || start_y == end_y {
        return true;
    }
    false
}

pub fn get_straight_overlap(lines: Vec<((i32, i32), (i32, i32))>) -> usize {
    let mut seen_points: HashSet<(i32, i32)> = HashSet::new();
    let mut points_seen_twice: HashSet<(i32, i32)> = HashSet::new();

    for line in lines.iter() {
        let (start, end) = *line;
        if is_straight(start, end) {
            for point in get_line(start, end).iter() {
                if seen_points.contains(point) {
                    points_seen_twice.insert(*point);
                }
                seen_points.insert(*point);
            }
        }
    }
    points_seen_twice.len()
}

pub fn get_all_overlap(lines: Vec<((i32, i32), (i32, i32))>) -> usize {
    let mut seen_points: HashSet<(i32, i32)> = HashSet::new();
    let mut points_seen_twice: HashSet<(i32, i32)> = HashSet::new();

    for line in lines.iter() {
        let (start, end) = *line;
        for point in get_line(start, end).iter() {
            if seen_points.contains(point) {
                points_seen_twice.insert(*point);
            }
            seen_points.insert(*point);
        }
    }
    points_seen_twice.len()
}

#[cfg(test)]
mod test {
    use super::{get_all_overlap, get_line, get_straight_overlap, is_straight};

    #[test]
    fn test_get_line() {
        // straight lines
        let mut first_answer = get_line((1, 1), (1, 3));
        first_answer.sort();
        assert_eq!(vec![(1, 1), (1, 2), (1, 3)], first_answer);

        let mut second_answer = get_line((9, 7), (7, 7));
        second_answer.sort();
        assert_eq!(vec![(7, 7), (8, 7), (9, 7)], second_answer);

        // diagonal lines
        let mut third_answer = get_line((1, 1), (3, 3));
        third_answer.sort();
        assert_eq!(vec![(1, 1), (2, 2), (3, 3)], third_answer);

        let mut fourth_answer = get_line((9, 7), (7, 9));
        fourth_answer.sort();
        assert_eq!(vec![(7, 9), (8, 8), (9, 7)], fourth_answer);
    }

    #[test]
    fn test_is_straight() {
        assert_eq!(true, is_straight((1, 1), (1, 3)));
        assert_eq!(false, is_straight((2, 1), (1, 3)));
    }

    #[test]
    fn test_straight_overlap() {
        let vents = vec![
            ((0, 9), (5, 9)),
            ((8, 0), (0, 8)),
            ((9, 4), (3, 4)),
            ((2, 2), (2, 1)),
            ((7, 0), (7, 4)),
            ((6, 4), (2, 0)),
            ((0, 9), (2, 9)),
            ((3, 4), (1, 4)),
            ((0, 0), (8, 8)),
            ((5, 5), (8, 2)),
        ];

        assert_eq!(5, get_straight_overlap(vents));
    }

    #[test]
    fn test_all_overlap() {
        let vents = vec![
            ((0, 9), (5, 9)),
            ((8, 0), (0, 8)),
            ((9, 4), (3, 4)),
            ((2, 2), (2, 1)),
            ((7, 0), (7, 4)),
            ((6, 4), (2, 0)),
            ((0, 9), (2, 9)),
            ((3, 4), (1, 4)),
            ((0, 0), (8, 8)),
            ((5, 5), (8, 2)),
        ];

        assert_eq!(12, get_all_overlap(vents));
    }
}
