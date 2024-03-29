use project_euler::utils;

pub fn is_prime(c: &mut criterion::Criterion) {
    c.bench_function("is_prime", |b| {
        b.iter(|| (0..10i64.pow(5)).map(utils::is_prime).count())
    });
}

pub fn sieve_of_atkin(c: &mut criterion::Criterion) {
    c.bench_function("sieve_of_atkin", |b| {
        b.iter(|| utils::SieveOfAtkin::new(10usize.pow(6)))
    });
}

criterion::criterion_group!(benches, is_prime, sieve_of_atkin);
criterion::criterion_main!(benches);
