pub mod solutions;
pub mod utils;

/// Execute the solution (if available) of the specified problem. Also measure
/// its running time.
///
/// * `problem_number`
///
/// -> Flag indicating whether the solution is available.
fn solve_and_time_one(problem_number: i32) -> bool {
    let solve = match problem_number {
        1 => solutions::multiples_of_3_or_5::solve,
        2 => solutions::even_fibonacci_numbers::solve,
        3 => solutions::largest_prime_factor::solve,
        4 => solutions::largest_palindrome_product::solve,
        5 => solutions::smallest_multiple::solve,
        6 => solutions::sum_square_difference::solve,
        7 => solutions::ten_thousand_and_first_prime::solve,
        8 => solutions::largest_product_in_a_series::solve,
        9 => solutions::special_pythagorean_triplet::solve,
        10 => solutions::summation_of_primes::solve,
        11 => solutions::largest_product_in_a_grid::solve,
        12 => solutions::highly_divisible_triangular_number::solve,
        13 => solutions::large_sum::solve,
        14 => solutions::longest_collatz_sequence::solve,
        15 => solutions::lattice_paths::solve,
        16 => solutions::power_digit_sum::solve,
        17 => solutions::number_letter_counts::solve,
        18 => solutions::maximum_path_sum_i::solve,
        19 => solutions::counting_sundays::solve,
        20 => solutions::factorial_digit_sum::solve,
        21 => solutions::amicable_numbers::solve,
        22 => solutions::names_scores::solve,
        23 => solutions::non_abundant_sums::solve,
        24 => solutions::lexicographic_permutations::solve,
        25 => solutions::thousand_digit_fibonacci_number::solve,
        26 => solutions::reciprocal_cycles::solve,
        27 => solutions::quadratic_primes::solve,
        28 => solutions::number_spiral_diagonals::solve,
        29 => solutions::distinct_powers::solve,
        30 => solutions::digit_fifth_powers::solve,
        31 => solutions::coin_sums::solve,
        32 => solutions::pandigital_products::solve,
        33 => solutions::digit_cancelling_fractions::solve,
        34 => solutions::digit_factorials::solve,
        35 => solutions::circular_primes::solve,
        36 => solutions::double_base_palindromes::solve,
        37 => solutions::truncatable_primes::solve,
        38 => solutions::pandigital_multiples::solve,
        39 => solutions::integer_right_triangles::solve,
        40 => solutions::champernownes_constant::solve,
        41 => solutions::pandigital_prime::solve,
        42 => solutions::coded_triangle_numbers::solve,
        43 => solutions::sub_string_divisibility::solve,
        44 => solutions::pentagon_numbers::solve,
        67 => solutions::maximum_path_sum_ii::solve,
        _ => return false,
    };
    let now = std::time::Instant::now();
    let result = solve();
    let elapsed = now.elapsed();
    println!(
        "{:>3} [{:>9.3} ms] {}",
        problem_number,
        elapsed.as_micros() as f64 / 1000.0,
        result
    );
    true
}

/// Execute the solutions of all available problems, measuring their running
/// times.
fn solve_and_time_all() {
    for problem_number in 1.. {
        if !solve_and_time_one(problem_number) {
            break;
        }
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() <= 1 {
        solve_and_time_all();
        return;
    }
    for arg in &args[1..] {
        match arg.parse() {
            Ok(problem_number) => {
                if solve_and_time_one(problem_number) {
                    continue;
                }
            }
            Err(_) => (),
        }
        eprintln!(
            "Problem {} does not exist or its solution is not implemented.",
            arg
        );
    }
}
