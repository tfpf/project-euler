

/// Execute the solution of the specified problem.
///
/// * problem_number
fn solve(problem_number: i32) {
    match problem_number {
        1 => multiples_of_3_or_5();
    }
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
