use std::collections::{HashSet, VecDeque};

pub fn sum_low_points(pts: Vec<Vec<usize>>) -> usize {
    let mut sum = 0;
    for (x, row) in pts.iter().enumerate() {
        for (y, elt) in row.iter().enumerate() {
            let adjacent_points = get_adjacent(x, y, &pts);
            if get_values(&adjacent_points, &pts).iter().all(|pt| pt > elt) {
                sum += elt + 1;
            }
        }
    }
    sum
}

fn get_low_points(pts: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let mut low_pts: Vec<(usize, usize)> = Vec::new();

    for (x, row) in pts.iter().enumerate() {
        for (y, elt) in row.iter().enumerate() {
            let adjacent_points = get_adjacent(x, y, &pts);
            if get_values(&adjacent_points, &pts).iter().all(|pt| pt > elt) {
                low_pts.push((x, y));
            }
        }
    }
    low_pts
}

fn get_basin_size(pts: &Vec<Vec<usize>>, low_pt: (usize, usize)) -> usize {
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut to_process: VecDeque<(usize, usize)> = VecDeque::from_iter([low_pt]);

    while !to_process.is_empty() {
        let (x, y) = to_process.pop_front().unwrap();
        if seen.contains(&(x, y)) {
            continue;
        }

        if pts[x][y] == 9 {
            continue;
        }

        seen.insert((x, y));

        let neighbors: Vec<(usize, usize)> = get_adjacent(x, y, pts);
        for &pt in neighbors.iter() {
            if !seen.contains(&pt) {
                to_process.push_back(pt);
            }
        }
    }
    seen.len()
}

pub fn find_basins(pts: Vec<Vec<usize>>) -> usize {
    //let mut basin_sizes: Vec<usize> = Vec::new();
    let low_pts = get_low_points(&pts);

    let mut basin_sizes: Vec<usize> = low_pts.iter().map(|&low_pt| get_basin_size(&pts, low_pt)).collect(); 
    basin_sizes.sort_unstable();
    basin_sizes
        .iter()
        .rev()
        .take(3)
        .copied()
        .product()
}

fn get_adjacent(x: usize, y: usize, pts: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let mut pos: Vec<(usize, usize)> = Vec::new();
    // above
    if x > 0 {
        pos.push((x - 1, y));
    }
    // below
    if x < pts.len() - 1 {
        pos.push((x + 1, y));
    }
    // left
    if y > 0 {
        pos.push((x, y - 1));
    }
    // right
    if y < pts[0].len() - 1 {
        pos.push((x, y + 1));
    }
    pos
}

fn get_values(positions: &Vec<(usize, usize)>, pts: &Vec<Vec<usize>>) -> Vec<usize> {
    positions
        .iter()
        .map(|&position| {
            let (x, y) = position;
            pts[x][y]
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::{find_basins, sum_low_points};

    #[test]
    fn test_low_points() {
        let pts = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];

        assert_eq!(15, sum_low_points(pts));
    }

    #[test]
    fn test_find_basins() {
        let pts = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];

        assert_eq!(1134, find_basins(pts));
    }
}
