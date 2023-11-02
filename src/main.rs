pub mod solutions;
pub mod utils;

/// Execute the solution of the specified problem.
///
/// * `problem_number`
fn solve(problem_number: i32) {
    match problem_number {
        1 => solutions::multiples_of_3_or_5::solve(),
        2 => solutions::even_fibonacci_numbers::solve(),
        3 => solutions::largest_prime_factor::solve(),
        4 => solutions::largest_palindromic_product::solve(),
        5 => solutions::smallest_multiple::solve(),
        6 => solutions::sum_square_difference::solve(),
        7 => solutions::ten_thousand_and_first_prime::solve(),
        8 => solutions::largest_product_in_a_series::solve(),
        9 => solutions::special_pythagorean_triplet::solve(),
        10 => solutions::summation_of_primes::solve(),
        11 => solutions::largest_product_in_a_grid::solve(),
        12 => solutions::highly_divisible_triangular_number::solve(),
        13 => solutions::large_sum::solve(),
        14 => solutions::longest_collatz_sequence::solve(),
        15 => solutions::lattice_paths::solve(),
        16 => solutions::power_digit_sum::solve(),
        17 => solutions::number_letter_counts::solve(),
        18 => solutions::maximum_path_sum_i::solve(),
        19 => solutions::counting_sundays::solve(),
        20 => solutions::factorial_digit_sum::solve(),
        _ => (),
    };
}

/// Main function.
fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let problem_number = match args.get(1) {
        Some(pn) => pn,
        _ => {
            println!("Problem number not specified.");
            return;
        }
    };
    let problem_number = match problem_number.parse::<i32>() {
        Ok(pn) if pn > 0 => pn,
        _ => {
            println!("Problem number is invalid.");
            return;
        }
    };
    solve(problem_number);
}
