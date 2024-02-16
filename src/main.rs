#![deny(clippy::all)]
#![allow(clippy::len_without_is_empty)]
#![allow(clippy::manual_try_fold)]
#![allow(clippy::match_overlapping_arm)]
#![allow(clippy::new_without_default)]

use std::io::Write;

pub mod solutions;
pub mod utils;

#[cfg(test)]
mod tests;

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
        45 => solutions::triangular_pentagonal_and_hexagonal::solve,
        46 => solutions::goldbachs_other_conjecture::solve,
        47 => solutions::distinct_primes_factors::solve,
        48 => solutions::self_powers::solve,
        49 => solutions::prime_permutations::solve,
        50 => solutions::consecutive_prime_sum::solve,
        51 => solutions::prime_digit_replacements::solve,
        52 => solutions::permuted_multiples::solve,
        53 => solutions::combinatoric_selections::solve,
        54 => solutions::poker_hands::solve,
        55 => solutions::lychrel_numbers::solve,
        56 => solutions::powerful_digit_sum::solve,
        57 => solutions::square_root_convergents::solve,
        58 => solutions::spiral_primes::solve,
        59 => solutions::xor_decryption::solve,
        61 => solutions::cyclical_figurate_numbers::solve,
        62 => solutions::cubic_permutations::solve,
        63 => solutions::powerful_digit_counts::solve,
        64 => solutions::odd_period_square_roots::solve,
        65 => solutions::convergents_of_e::solve,
        66 => solutions::diophantine_equation::solve,
        67 => solutions::maximum_path_sum_ii::solve,
        68 => solutions::magic_5_gon_ring::solve,
        69 => solutions::totient_maximum::solve,
        71 => solutions::ordered_fractions::solve,
        74 => solutions::digit_factorial_chains::solve,
        75 => solutions::singular_integer_right_triangles::solve,
        76 => solutions::counting_summations::solve,
        77 => solutions::prime_summations::solve,
        81 => solutions::path_sum_two_ways::solve,
        85 => solutions::counting_rectangles::solve,
        87 => solutions::prime_power_triples::solve,
        92 => solutions::square_digit_chains::solve,
        97 => solutions::large_non_mersenne_prime::solve,
        99 => solutions::largest_exponential::solve,
        _ => return false,
    };
    let now = std::time::Instant::now();
    let result = solve();
    let elapsed = now.elapsed().as_micros() as f64 / 1000.0;
    println!("{:>3} [{:>9.3} ms] {}", problem_number, elapsed, result);
    true
}

/// Execute the solutions of all available problems, measuring their running
/// times.
fn solve_and_time_all() {
    for problem_number in 1..=100 {
        solve_and_time_one(problem_number);
    }
}

/// Write a file with the given contents.
///
/// * `fname` - File name.
/// * `append` - Whether to append to an existing file or create a new file.
/// * `contents` - What to write in the file.
fn add_skel(fname: &str, append: bool, contents: &str) {
    let mut fhandle = std::fs::OpenOptions::new()
        .append(append)
        .create_new(!append)
        .write(true)
        .open(fname)
        .unwrap();
    writeln!(fhandle, "{}", contents).unwrap();
}

/// Perform minimal setup (providing a skeleton) to start solving a new
/// problem.
///
/// * `problem_number`
fn add_skels(problem_number: i32) {
    let url = format!("https://projecteuler.net/problem={}", problem_number);
    let output = std::process::Command::new("curl").args([&url]).output().unwrap();
    let html = std::str::from_utf8(&output.stdout).unwrap();
    let begin = html.find("<title>").unwrap() + 7;
    let begin = begin + html[begin..].find(' ').unwrap() + 1;
    let end = begin + html[begin..].find(" - Project Euler").unwrap();
    let title = html[begin..end]
        .chars()
        .filter_map(|c| match c {
            '0'..='9' | 'A'..='Z' | 'a'..='z' => Some(c.to_ascii_lowercase()),
            ' ' | '-' | '_' => Some('_'),
            _ => None,
        })
        .collect::<String>();

    add_skel(&format!("src/solutions/{}.rs", title), false, "pub fn solve()->i64{0}");
    add_skel("src/solutions.rs", true, &format!("pub mod {};", title));
    add_skel(
        "README.md",
        true,
        &format!(
            "|[{}]({})|[`{}.rs`](src/solutions/{}.rs)|",
            problem_number, url, title, title
        ),
    );
    add_skel(
        "src/main.rs",
        true,
        &format!("        {} => solutions::{}::solve,", problem_number, title),
    );
    std::process::Command::new("git")
        .args(["checkout", "-b", &format!("p{}", problem_number)])
        .output()
        .unwrap();
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() <= 1 {
        solve_and_time_all();
        return;
    }
    if args.len() == 3 && args[1] == "--add" {
        if let Ok(problem_number) = args[2].parse() {
            add_skels(problem_number);
            return;
        }
    }
    for arg in &args[1..] {
        if !arg.parse().is_ok_and(solve_and_time_one) {
            eprintln!("Problem {} does not exist or its solution is not implemented.", arg);
        }
    }
}
