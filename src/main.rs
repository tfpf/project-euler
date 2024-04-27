use project_euler::solutions::*;
use std::io::Write;

/// Execute the solution (if available) of the specified problem. Also measure
/// its running time.
///
/// * `problem_number`
///
/// -> Flag indicating whether the solution is available.
fn solve_and_time_one(problem_number: usize) -> bool {
    let solve = match problem_number {
        1 => multiples_of_3_or_5::solve,
        2 => even_fibonacci_numbers::solve,
        3 => largest_prime_factor::solve,
        4 => largest_palindrome_product::solve,
        5 => smallest_multiple::solve,
        6 => sum_square_difference::solve,
        7 => ten_thousand_and_first_prime::solve,
        8 => largest_product_in_a_series::solve,
        9 => special_pythagorean_triplet::solve,
        10 => summation_of_primes::solve,
        11 => largest_product_in_a_grid::solve,
        12 => highly_divisible_triangular_number::solve,
        13 => large_sum::solve,
        14 => longest_collatz_sequence::solve,
        15 => lattice_paths::solve,
        16 => power_digit_sum::solve,
        17 => number_letter_counts::solve,
        18 => maximum_path_sum_i::solve,
        19 => counting_sundays::solve,
        20 => factorial_digit_sum::solve,
        21 => amicable_numbers::solve,
        22 => names_scores::solve,
        23 => non_abundant_sums::solve,
        24 => lexicographic_permutations::solve,
        25 => thousand_digit_fibonacci_number::solve,
        26 => reciprocal_cycles::solve,
        27 => quadratic_primes::solve,
        28 => number_spiral_diagonals::solve,
        29 => distinct_powers::solve,
        30 => digit_fifth_powers::solve,
        31 => coin_sums::solve,
        32 => pandigital_products::solve,
        33 => digit_cancelling_fractions::solve,
        34 => digit_factorials::solve,
        35 => circular_primes::solve,
        36 => double_base_palindromes::solve,
        37 => truncatable_primes::solve,
        38 => pandigital_multiples::solve,
        39 => integer_right_triangles::solve,
        40 => champernownes_constant::solve,
        41 => pandigital_prime::solve,
        42 => coded_triangle_numbers::solve,
        43 => sub_string_divisibility::solve,
        44 => pentagon_numbers::solve,
        45 => triangular_pentagonal_and_hexagonal::solve,
        46 => goldbachs_other_conjecture::solve,
        47 => distinct_primes_factors::solve,
        48 => self_powers::solve,
        49 => prime_permutations::solve,
        50 => consecutive_prime_sum::solve,
        51 => prime_digit_replacements::solve,
        52 => permuted_multiples::solve,
        53 => combinatoric_selections::solve,
        54 => poker_hands::solve,
        55 => lychrel_numbers::solve,
        56 => powerful_digit_sum::solve,
        57 => square_root_convergents::solve,
        58 => spiral_primes::solve,
        59 => xor_decryption::solve,
        61 => cyclical_figurate_numbers::solve,
        62 => cubic_permutations::solve,
        63 => powerful_digit_counts::solve,
        64 => odd_period_square_roots::solve,
        65 => convergents_of_e::solve,
        66 => diophantine_equation::solve,
        67 => maximum_path_sum_ii::solve,
        68 => magic_5_gon_ring::solve,
        69 => totient_maximum::solve,
        71 => ordered_fractions::solve,
        72 => counting_fractions::solve,
        74 => digit_factorial_chains::solve,
        75 => singular_integer_right_triangles::solve,
        76 => counting_summations::solve,
        77 => prime_summations::solve,
        78 => coin_partitions::solve,
        81 => path_sum_two_ways::solve,
        85 => counting_rectangles::solve,
        87 => prime_power_triples::solve,
        92 => square_digit_chains::solve,
        97 => large_non_mersenne_prime::solve,
        99 => largest_exponential::solve,
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
macro_rules! add_skel {
    ($fname:expr, $append:literal, $($contents:expr),+) => {
        let mut open_options = std::fs::OpenOptions::new();
        let mut fhandle = open_options.append($append).create_new(!$append).write(true).open($fname).unwrap();
        writeln!(fhandle, $($contents),+).unwrap();
    };
}

/// Perform minimal setup (providing a skeleton) to start solving a new
/// problem.
///
/// * `problem_number`
fn add_skels(problem_number: usize) {
    let url = "https://projecteuler.net/minimal=problems";
    let output = std::process::Command::new("curl").arg(url).output().unwrap();
    let output = std::str::from_utf8(&output.stdout).unwrap();
    let line = output.lines().nth(problem_number).unwrap();
    let title = line.split("##").nth(1).unwrap();
    let title = title
        .chars()
        .filter_map(|c| match c {
            '0'..='9' | 'A'..='Z' | 'a'..='z' => Some(c.to_ascii_lowercase()),
            ' ' | '-' | '_' => Some('_'),
            _ => None,
        })
        .collect::<String>();

    add_skel!(
        &format!("src/solutions/{}.rs", title),
        false,
        "pub fn solve()->i64{{0}}"
    );
    add_skel!("src/solutions.rs", true, "pub mod {};", title);
    add_skel!(
        "README.md",
        true,
        "|[{}]({})|[`{}.rs`](src/solutions/{}.rs)|",
        problem_number,
        url,
        title,
        title
    );
    add_skel!("src/main.rs", true, "        {} => {}::solve,", problem_number, title);
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
    // If a negative number is provided, add skeleton code for that problem.
    // This is a convenience for me, and not the expected usage.
    if args[1].starts_with('-') {
        if let Ok(problem_number) = args[1][1..].parse() {
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
