use project_euler::utils;

#[divan::bench]
fn sieve_of_atkin() {
    utils::SieveOfAtkin::new(100000000);
}

fn main() {
    divan::main();
}
