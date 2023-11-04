pub mod solutions;
pub mod utils;

/// Execute the solution (if available) of the specified problem.
///
/// * `problem_number`
///
/// -> Flag indicating whether the solution is available.
fn solve(problem_number: i32) -> bool {
    match problem_number {
        1 => solutions::multiples_of_3_or_5::solve(),
        2 => solutions::even_fibonacci_numbers::solve(),
        3 => solutions::largest_prime_factor::solve(),
        4 => solutions::largest_palindrome_product::solve(),
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
        21 => solutions::amicable_numbers::solve(),
        22 => solutions::names_scores::solve(),
        23 => solutions::non_abundant_sums::solve(),
        24 => solutions::lexicographic_permutations::solve(),
        25 => solutions::thousand_digit_fibonacci_number::solve(),
        _ => return false,
    };
    true
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Problem number not specified.");
        return;
    }
    let problem_number = match args[1].parse::<i32>() {
        Ok(pn) if pn >= 0 => pn,
        _ => {
            eprintln!("Problem number is invalid.");
            return;
        }
    };
    if problem_number == 0 {
        for pn in 1.. {
            if !solve(pn) {
                break;
            }
        }
    } else {
        solve(problem_number);
    }
}
