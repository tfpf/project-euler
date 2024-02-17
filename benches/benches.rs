use project_euler::utils;

pub fn sieve_of_atkin(c: &mut criterion::Criterion) {
    c.bench_function("sieve_of_atkin", |b| b.iter(|| utils::SieveOfAtkin::new(1000000)));
}

criterion::criterion_group!(benches, sieve_of_atkin);
criterion::criterion_main!(benches);
