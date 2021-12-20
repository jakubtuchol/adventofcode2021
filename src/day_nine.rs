use std::collections::HashSet;

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

pub fn find_basins(pts: Vec<Vec<usize>>) -> usize {
    let mut basin_sizes: Vec<usize> = Vec::new();

    for (x, row) in pts.iter().enumerate() {
        for (y, &elt) in row.iter().enumerate() {
            let adjacent = get_adjacent(x, y, &pts);

            if get_values(&adjacent, &pts).iter().all(|&pt| pt > elt) {
                let mut basin_pts: HashSet<(usize, usize)> = HashSet::new();
                // build map of adjacent
                basin_pts.insert((x, y));

                let mut to_process: Vec<(usize, usize)> = adjacent.clone();

                while !to_process.is_empty() {
                    let (next_x, next_y) = to_process.pop().unwrap();
                    if basin_pts.contains(&(next_x, next_y)) {
                        continue;
                    }
                    let cur_elt = pts[next_x][next_y];
                    let cur_adjacent = get_adjacent(next_x, next_y, &pts);

                    let mut not_processed: Vec<(usize, usize)> = cur_adjacent
                        .into_iter()
                        .filter(|elt| !basin_pts.contains(elt))
                        .collect();

                    if not_processed.is_empty() {
                        
                    }

                    if !not_processed.is_empty()
                        && get_values(&not_processed, &pts)
                            .iter()
                            .all(|&x| x > cur_elt)
                    {
                        basin_pts.insert((next_x, next_y));
                        to_process.append(&mut not_processed);
                    }
                }
                basin_sizes.push(basin_pts.len());
            }
        }
    }
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
