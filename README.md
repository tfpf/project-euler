# Project Euler
Trying my hand at [Project Euler](https://projecteuler.net) as I stumble along learning Rust. I shall only add the
solutions to the first hundred problems here with the intention being to showcase whatever useful data structures I
build along the way. This is permitted according to the Project Euler guidelines.

![style](https://github.com/tfpf/project-euler/actions/workflows/style.yml/badge.svg)
![tests](https://github.com/tfpf/project-euler/actions/workflows/tests.yml/badge.svg)

To solve, say, problem 16, enter the following command.

```sh
cargo r 16
```

You can specify 0 to sequentially solve all problems for which I have written solutions.

```sh
cargo r 0
```

Most solutions are rather concise; the heavy lifting is done in [`src/utils.rs`](src/utils.rs). This highlights the
intent of the code by hiding confounding implementation details.

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
