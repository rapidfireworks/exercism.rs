use anagram::anagrams_for;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
  let word = "allergy";

  let inputs = [
    "gallery",
    "ballerina",
    "regally",
    "clergy",
    "largely",
    "leading",
  ];

  c.bench_function("test_multiple_anagrams", |b| {
    b.iter(|| anagrams_for(word, &inputs))
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
