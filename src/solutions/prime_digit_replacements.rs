use crate::utils;

/// Find an 8-family of prime numbers which have the same digits, except for
/// those in some fixed positions; said positions have the same digit.
///
/// The number of fixed positions must be 3, 6 or 9. Otherwise, at least one
/// of the 8 numbers in the family will be divisible by 3. We get lucky with
/// assuming that it is 3.
///
/// -> Smallest member of the family.
fn prime_digit_replacements() -> i64 {
    const LIMIT: usize = 1000000;
    let sieve = utils::SieveOfAtkin::new(LIMIT);
    for prime in sieve.iter() {
        // Break the number into its digits. Note the position/positions each
        // appears at.
        let mut digitpos: [Vec<usize>; 3] = [vec![], vec![], vec![]];
        for (position, digit) in utils::Digits::new(prime)
            .enumerate()
            // The units digit cannot be replaced, since we would then have
            // even numbers.
            .skip(1)
            // If the digit is 3 or more, we cannot have an 8-family.
            .filter(|&(_, digit)| digit <= 2)
        {
            digitpos[digit as usize].push(position);
        }

        // Which digit can we replace?
        for (digit, positions) in digitpos.iter().enumerate() {
            match positions.len() {
                3 | 6 | 9 => (),
                _ => continue,
            }

            // Replace all occurrences of this digit with successive digits.
            // Find the number which, upon repeatedly adding to the current
            // prime, will have said effect.
            let jump = positions
                .iter()
                .map(|&position| 10usize.pow(position as u32))
                .sum::<usize>();
            // How many times should it be added? Until the replaced digit
            // becomes 9.
            let candidates = 9 - digit as usize;

            if (prime as usize..)
                .step_by(jump)
                // The first number is already known to be prime, so ignore it.
                // Out of the remaining numbers, 7 should be prime.
                .skip(1)
                .take(candidates)
                .filter(|&num| sieve.is_prime(num))
                .count()
                == 7
            {
                return prime;
            }
        }
    }
    unreachable!();
}

pub fn solve() -> i64 {
    let result = prime_digit_replacements();

    assert_eq!(result, 121313);
    result
}
