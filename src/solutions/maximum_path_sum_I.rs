/// Calculate the maximum path sum in the triangular two-dimensional vector.
///
/// * `triangle`
/// * `row` - Row index of the cell where the path starts.
/// * `col` - Column index of the cell where the path starts.
///
/// -> Maximum path sum over all paths which end at the last row.
fn max_sum(triangle: &[Vec<i32>; 15], row: usize, col: usize) -> i32
{
    if row >= 15
    {
        return 0;
    }

    // The problem statement specifically says I can brute-force it. I'll worry
    // about dynamic programming later.
    let go_left = max_sum(triangle, row + 1, col);
    let go_right = max_sum(triangle, row + 1, col + 1);
    triangle[row][col] + std::cmp::max(go_left, go_right)
}

/// Main function.
fn main()
{
    let triangle = [
        vec![75],
        vec![95,  64],
        vec![17,  47,  82],
        vec![18,  35,  87,  10],
        vec![20,  04,  82,  47,  65],
        vec![19,  01,  23,  75,  03,  34],
        vec![88,  02,  77,  73,  07,  63,  67],
        vec![99,  65,  04,  28,  06,  16,  70,  92],
        vec![41,  41,  26,  56,  83,  40,  80,  70,  33],
        vec![41,  48,  72,  33,  47,  32,  37,  16,  94,  29],
        vec![53,  71,  44,  65,  25,  43,  91,  52,  97,  51,  14],
        vec![70,  11,  33,  28,  77,  73,  17,  78,  39,  68,  17,  57],
        vec![91,  71,  52,  38,  17,  14,  91,  43,  58,  50,  27,  29,  48],
        vec![63,  66,  04,  68,  89,  53,  67,  30,  73,  16,  69,  87,  40,  31],
        vec![04,  62,  98,  27,  23,  09,  70,  98,  73,  93,  38,  53,  60,  04,  23],
    ];
    let result = max_sum(&triangle, 0, 0);
    println!("{}", result);

    assert_eq!(result, 1074);
}
