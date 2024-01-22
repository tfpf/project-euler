# Project Euler
Trying my hand at [Project Euler](https://projecteuler.net) as I stumble along learning Rust. I shall only add the
solutions to the first hundred problems here with the intention being to showcase whatever useful data structures I
build along the way. This is permitted according to the Project Euler guidelines.

[![style](https://github.com/tfpf/project-euler/actions/workflows/style.yml/badge.svg)](https://github.com/tfpf/project-euler/actions/workflows/style.yml)
[![lint](https://github.com/tfpf/project-euler/actions/workflows/lint.yml/badge.svg)](https://github.com/tfpf/project-euler/actions/workflows/lint.yml)
[![sanity](https://github.com/tfpf/project-euler/actions/workflows/sanity.yml/badge.svg)](https://github.com/tfpf/project-euler/actions/workflows/sanity.yml)
[![tests](https://github.com/tfpf/project-euler/actions/workflows/tests.yml/badge.svg)](https://github.com/tfpf/project-euler/actions/workflows/tests.yml)

To solve, say, problem 16, enter the following command.

```sh
cargo r 16
```

Run it without arguments to sequentially solve all problems for which I have written solutions.

```sh
cargo r
```

Most solutions are rather concise; the heavy lifting is done in [`src/utils.rs`](src/utils.rs). This highlights the
intent of the code by hiding confounding implementation details.

<p align="center">
 <img src="res/certified_human.svg" />
</p>

No part of the code in this repository has been written by or in consultation with artificial intelligence chatbots
such as (but not limited to) Bard and ChatGPT.

## Problems and Solutions
|Problem|Solution|
|-:|:-:|
|[1](https://projecteuler.net/problem=1)|[`multiples_of_3_or_5.rs`](src/solutions/multiples_of_3_or_5.rs)|
|[2](https://projecteuler.net/problem=2)|[`even_fibonacci_numbers.rs`](src/solutions/even_fibonacci_numbers.rs)|
|[3](https://projecteuler.net/problem=3)|[`largest_prime_factor.rs`](src/solutions/largest_prime_factor.rs)|
|[4](https://projecteuler.net/problem=4)|[`largest_palindrome_product.rs`](src/solutions/largest_palindrome_product.rs)|
|[5](https://projecteuler.net/problem=5)|[`smallest_multiple.rs`](src/solutions/smallest_multiple.rs)|
|[6](https://projecteuler.net/problem=6)|[`sum_square_difference.rs`](src/solutions/sum_square_difference.rs)|
|[7](https://projecteuler.net/problem=7)|[`ten_thousand_and_first_prime.rs`](src/solutions/ten_thousand_and_first_prime.rs)|
|[8](https://projecteuler.net/problem=8)|[`largest_product_in_a_series.rs`](src/solutions/largest_product_in_a_series.rs)|
|[9](https://projecteuler.net/problem=9)|[`special_pythagorean_triplet.rs`](src/solutions/special_pythagorean_triplet.rs)|
|[10](https://projecteuler.net/problem=10)|[`summation_of_primes.rs`](src/solutions/summation_of_primes.rs)|
|[11](https://projecteuler.net/problem=11)|[`largest_product_in_a_grid.rs`](src/solutions/largest_product_in_a_grid.rs)|
|[12](https://projecteuler.net/problem=12)|[`highly_divisible_triangular_number.rs`](src/solutions/highly_divisible_triangular_number.rs)|
|[13](https://projecteuler.net/problem=13)|[`large_sum.rs`](src/solutions/large_sum.rs)|
|[14](https://projecteuler.net/problem=14)|[`longest_collatz_sequence.rs`](src/solutions/longest_collatz_sequence.rs)|
|[15](https://projecteuler.net/problem=15)|[`lattice_paths.rs`](src/solutions/lattice_paths.rs)|
|[16](https://projecteuler.net/problem=16)|[`power_digit_sum.rs`](src/solutions/power_digit_sum.rs)|
|[17](https://projecteuler.net/problem=17)|[`number_letter_counts.rs`](src/solutions/number_letter_counts.rs)|
|[18](https://projecteuler.net/problem=18)|[`maximum_path_sum_i.rs`](src/solutions/maximum_path_sum_i.rs)|
|[19](https://projecteuler.net/problem=19)|[`counting_sundays.rs`](src/solutions/counting_sundays.rs)|
|[20](https://projecteuler.net/problem=20)|[`factorial_digit_sum.rs`](src/solutions/factorial_digit_sum.rs)|
|[21](https://projecteuler.net/problem=21)|[`amicable_numbers.rs`](src/solutions/amicable_numbers.rs)|
|[22](https://projecteuler.net/problem=22)|[`names_scores.rs`](src/solutions/names_scores.rs)|
|[23](https://projecteuler.net/problem=23)|[`non_abundant_sums.rs`](src/solutions/non_abundant_sums.rs)|
|[24](https://projecteuler.net/problem=24)|[`lexicographic_permutations.rs`](src/solutions/lexicographic_permutations.rs)|
|[25](https://projecteuler.net/problem=25)|[`thousand_digit_fibonacci_number.rs`](src/solutions/thousand_digit_fibonacci_number.rs)|
|[26](https://projecteuler.net/problem=26)|[`reciprocal_cycles.rs`](src/solutions/reciprocal_cycles.rs)|
|[27](https://projecteuler.net/problem=27)|[`quadratic_primes.rs`](src/solutions/quadratic_primes.rs)|
|[28](https://projecteuler.net/problem=28)|[`number_spiral_diagonals.rs`](src/solutions/number_spiral_diagonals.rs)|
|[29](https://projecteuler.net/problem=29)|[`distinct_powers.rs`](src/solutions/distinct_powers.rs)|
|[30](https://projecteuler.net/problem=30)|[`digit_fifth_powers.rs`](src/solutions/digit_fifth_powers.rs)|
|[31](https://projecteuler.net/problem=31)|[`coin_sums.rs`](src/solutions/coin_sums.rs)|
|[32](https://projecteuler.net/problem=32)|[`pandigital_products.rs`](src/solutions/pandigital_products.rs)|
|[33](https://projecteuler.net/problem=33)|[`digit_cancelling_fractions.rs`](src/solutions/digit_cancelling_fractions.rs)|
|[34](https://projecteuler.net/problem=34)|[`digit_factorials.rs`](src/solutions/digit_factorials.rs)|
|[35](https://projecteuler.net/problem=35)|[`circular_primes.rs`](src/solutions/circular_primes.rs)|
|[36](https://projecteuler.net/problem=36)|[`double_base_palindromes.rs`](src/solutions/double_base_palindromes.rs)|
|[37](https://projecteuler.net/problem=37)|[`truncatable_primes.rs`](src/solutions/truncatable_primes.rs)|
|[38](https://projecteuler.net/problem=38)|[`pandigital_multiples.rs`](src/solutions/pandigital_multiples.rs)|
|[39](https://projecteuler.net/problem=39)|[`integer_right_triangles.rs`](src/solutions/integer_right_triangles.rs)|
|[40](https://projecteuler.net/problem=40)|[`champernownes_constant.rs`](src/solutions/champernownes_constant.rs)|
|[41](https://projecteuler.net/problem=41)|[`pandigital_prime.rs`](src/solutions/pandigital_prime.rs)|
|[42](https://projecteuler.net/problem=42)|[`coded_triangle_numbers.rs`](src/solutions/coded_triangle_numbers.rs)|
|[43](https://projecteuler.net/problem=43)|[`sub_string_divisibility.rs`](src/solutions/sub_string_divisibility.rs)|
|[44](https://projecteuler.net/problem=44)|[`pentagon_numbers.rs`](src/solutions/pentagon_numbers.rs)|
|[45](https://projecteuler.net/problem=45)|[`triangular_pentagonal_and_hexagonal.rs`](src/solutions/triangular_pentagonal_and_hexagonal.rs)|
|[46](https://projecteuler.net/problem=46)|[`goldbachs_other_conjecture.rs`](src/solutions/goldbachs_other_conjecture.rs)|
|[47](https://projecteuler.net/problem=47)|[`distinct_primes_factors.rs`](src/solutions/distinct_primes_factors.rs)|
|[48](https://projecteuler.net/problem=48)|[`self_powers.rs`](src/solutions/self_powers.rs)|
|[49](https://projecteuler.net/problem=49)|[`prime_permutations.rs`](src/solutions/prime_permutations.rs)|
|[50](https://projecteuler.net/problem=50)|[`consecutive_prime_sum.rs`](src/solutions/consecutive_prime_sum.rs)|
|[52](https://projecteuler.net/problem=52)|[`permuted_multiples.rs`](src/solutions/permuted_multiples.rs)|
|[53](https://projecteuler.net/problem=53)|[`combinatoric_selections.rs`](src/solutions/combinatoric_selections.rs)|
|[54](https://projecteuler.net/problem=54)|[`poker_hands.rs`](src/solutions/poker_hands.rs)|
|[55](https://projecteuler.net/problem=55)|[`lychrel_numbers.rs`](src/solutions/lychrel_numbers.rs)|
|[56](https://projecteuler.net/problem=56)|[`powerful_digit_sum.rs`](src/solutions/powerful_digit_sum.rs)|
|[57](https://projecteuler.net/problem=57)|[`square_root_convergents.rs`](src/solutions/square_root_convergents.rs)|
|[58](https://projecteuler.net/problem=58)|[`spiral_primes.rs`](src/solutions/spiral_primes.rs)|
|[59](https://projecteuler.net/problem=59)|[`xor_decryption.rs`](src/solutions/xor_decryption.rs)|
|[62](https://projecteuler.net/problem=62)|[`cubic_permutations.rs`](src/solutions/cubic_permutations.rs)|
|[63](https://projecteuler.net/problem=63)|[`powerful_digit_counts.rs`](src/solutions/powerful_digit_counts.rs)|
|[64](https://projecteuler.net/problem=64)|[`odd_period_square_roots.rs`](src/solutions/odd_period_square_roots.rs)|
|[65](https://projecteuler.net/problem=65)|[`convergents_of_e.rs`](src/solutions/convergents_of_e.rs)|
|[67](https://projecteuler.net/problem=67)|[`maximum_path_sum_ii.rs`](src/solutions/maximum_path_sum_ii.rs)|
|[69](https://projecteuler.net/problem=69)|[`totient_maximum.rs`](src/solutions/totient_maximum.rs)|
|[74](https://projecteuler.net/problem=74)|[`digit_factorial_chains.rs`](src/solutions/digit_factorial_chains.rs)|
|[75](https://projecteuler.net/problem=75)|[`singular_integer_right_triangles.rs`](src/solutions/singular_integer_right_triangles.rs)|
|[81](https://projecteuler.net/problem=81)|[`path_sum_two_ways.rs`](src/solutions/path_sum_two_ways.rs)|
|[85](https://projecteuler.net/problem=85)|[`counting_rectangles.rs`](src/solutions/counting_rectangles.rs)|
|[87](https://projecteuler.net/problem=87)|[`prime_power_triples.rs`](src/solutions/prime_power_triples.rs)|
|[92](https://projecteuler.net/problem=92)|[`square_digit_chains.rs`](src/solutions/square_digit_chains.rs)|
|[97](https://projecteuler.net/problem=97)|[`large_non_mersenne_prime.rs`](src/solutions/large_non_mersenne_prime.rs)|
