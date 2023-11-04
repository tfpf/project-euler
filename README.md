# Project Euler
My solutions to some [Project Euler](https://projecteuler.net) problems, written as I stumble along learning Rust. I
shall only add the solutions to the first hundred problems here with the intention being to showcase whatever useful
data structures I build along the way. This is permitted according to the Project Euler guidelines.

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
|[4](https://projecteuler.net/problem=4)|[`largest_palindromic_product.rs`](src/solutions/largest_palindromic_product.rs)|
