pub mod amicable_numbers;
pub mod champernownes_constant;
pub mod circular_primes;
pub mod coded_triangle_numbers;
pub mod coin_partitions;
pub mod coin_sums;
pub mod combinatoric_selections;
pub mod consecutive_prime_sum;
pub mod convergents_of_e;
pub mod counting_fractions;
pub mod counting_rectangles;
pub mod counting_summations;
pub mod counting_sundays;
pub mod cubic_permutations;
pub mod cyclical_figurate_numbers;
pub mod digit_cancelling_fractions;
pub mod digit_factorial_chains;
pub mod digit_factorials;
pub mod digit_fifth_powers;
pub mod diophantine_equation;
pub mod distinct_powers;
pub mod distinct_primes_factors;
pub mod double_base_palindromes;
pub mod even_fibonacci_numbers;
pub mod factorial_digit_sum;
pub mod goldbachs_other_conjecture;
pub mod highly_divisible_triangular_number;
pub mod integer_right_triangles;
pub mod large_non_mersenne_prime;
pub mod large_sum;
pub mod largest_exponential;
pub mod largest_palindrome_product;
pub mod largest_prime_factor;
pub mod largest_product_in_a_grid;
pub mod largest_product_in_a_series;
pub mod lattice_paths;
pub mod lexicographic_permutations;
pub mod longest_collatz_sequence;
pub mod lychrel_numbers;
pub mod magic_5_gon_ring;
pub mod maximum_path_sum_i;
pub mod maximum_path_sum_ii;
pub mod multiples_of_3_or_5;
pub mod names_scores;
pub mod non_abundant_sums;
pub mod number_letter_counts;
pub mod number_spiral_diagonals;
pub mod odd_period_square_roots;
pub mod ordered_fractions;
pub mod pandigital_multiples;
pub mod pandigital_prime;
pub mod pandigital_products;
pub mod path_sum_two_ways;
pub mod pentagon_numbers;
pub mod permuted_multiples;
pub mod poker_hands;
pub mod power_digit_sum;
pub mod powerful_digit_counts;
pub mod powerful_digit_sum;
pub mod prime_digit_replacements;
pub mod prime_permutations;
pub mod prime_power_triples;
pub mod prime_summations;
pub mod quadratic_primes;
pub mod reciprocal_cycles;
pub mod self_powers;
pub mod singular_integer_right_triangles;
pub mod smallest_multiple;
pub mod special_pythagorean_triplet;
pub mod spiral_primes;
pub mod square_digit_chains;
pub mod square_root_convergents;
pub mod sub_string_divisibility;
pub mod sum_square_difference;
pub mod summation_of_primes;
pub mod ten_thousand_and_first_prime;
pub mod thousand_digit_fibonacci_number;
pub mod totient_maximum;
pub mod triangular_pentagonal_and_hexagonal;
pub mod truncatable_primes;
pub mod xor_decryption;

#[cfg(test)]
mod tests {
    #[test]
    fn all_solutions_assertions() {
        // Every solution should assert the equality of the expected and
        // observed answers.
        for dirent in std::fs::read_dir("src/solutions").unwrap() {
            let code = std::fs::read_to_string(dirent.unwrap().path()).unwrap();
            assert!(code.contains("assert_eq!"));
        }
    }
}
