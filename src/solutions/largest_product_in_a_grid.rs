use std::io::BufRead;

/// Create a vector of indices to iterate over.
///
/// * `idx` - Starting index.
/// * `delta` - Step to increment the index by.
///
/// -> Vector of indices. Empty if `delta` is not one of [-1, 0, 1].
fn create_vector(idx: usize, delta: isize) -> Vec<usize> {
    if delta == 1 {
        (idx..20).collect::<Vec<usize>>()
    } else if delta == -1 {
        (0..=idx).rev().collect::<Vec<usize>>()
    } else if delta == 0 {
        vec![idx; 20]
    } else {
        vec![]
    }
}

/// Find the largest product of four numbers in the grid while traversing it in
/// the manner described.
///
/// * `grid` - 2D array.
/// * `x` - Starting row index.
/// * `y` - Starting column index.
/// * `outer_dx` - How much to step the row index by in the outer loop.
/// * `outer_dy` - How much to step the column index by in the outer loop.
/// * `inner_dx` - How much to step the row index by in the inner loop.
/// * `inner_dy` - How much to step the column index by in the inner loop.
///
/// -> Largest product.
fn find_largest(
    grid: &[Vec<i32>],
    x: usize,
    y: usize,
    outer_dx: isize,
    outer_dy: isize,
    inner_dx: isize,
    inner_dy: isize,
) -> i32 {
    let mut result = -1;

    let outer_x = create_vector(x, outer_dx);
    let outer_y = create_vector(y, outer_dy);
    for (rstart, cstart) in outer_x.iter().zip(outer_y.iter()) {
        let inner_x = create_vector(*rstart, inner_dx);
        let inner_y = create_vector(*cstart, inner_dy);
        let mut product = 1;
        let mut zeros = 0;
        for (i, (_, _)) in inner_x.iter().zip(inner_y.iter()).enumerate() {
            // Divide first to avoid overflow.
            let outgoing = if i >= 4 {
                grid[inner_x[i - 4]][inner_y[i - 4]]
            } else {
                1
            };
            if outgoing == 0 {
                zeros -= 1;
            } else {
                product /= outgoing;
            }
            let incoming = grid[inner_x[i]][inner_y[i]];
            if incoming == 0 {
                zeros += 1;
            } else {
                product *= incoming;
            }
            if zeros == 0 && result < product {
                result = product;
            }
        }
    }
    result
}

pub fn solve() -> i64 {
    let fhandle = std::fs::File::open("res/solutions/largest_product_in_a_grid.txt").unwrap();
    let reader = std::io::BufReader::new(fhandle);
    let grid: Vec<Vec<i32>> = reader
        .lines()
        .map(|line| line.unwrap().split(' ').map(|s| s.parse().unwrap()).collect())
        .collect();

    let horizontal = find_largest(&grid, 0, 0, 1, 0, 0, 1);
    let vertical = find_largest(&grid, 0, 0, 0, 1, 1, 0);
    let slash_upper = find_largest(&grid, 0, 0, 0, 1, 1, -1);
    let slash_lower = find_largest(&grid, 1, 19, 1, 0, 1, -1);
    let backslash_upper = find_largest(&grid, 0, 19, 0, -1, 1, 1);
    let backslash_lower = find_largest(&grid, 1, 0, 1, 0, 1, 1);
    let maxima = [
        horizontal,
        vertical,
        slash_upper,
        slash_lower,
        backslash_upper,
        backslash_lower,
    ];
    let result = *maxima.iter().max().unwrap();

    assert_eq!(result, 70600674);
    result as i64
}
