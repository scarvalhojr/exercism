use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rayon::ThreadPoolBuilder;
use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    ThreadPoolBuilder::new()
        .num_threads(worker_count)
        .build()
        .unwrap()
        .install(|| {
            input
                .into_par_iter()
                .map(|line| {
                    let mut counters = HashMap::new();
                    for ch in line
                        .to_lowercase()
                        .chars()
                        .filter(|c| c.is_alphabetic())
                    {
                        counters.entry(ch).and_modify(|n| *n += 1).or_insert(1);
                    }
                    counters
                })
                .reduce(HashMap::new, |mut counters_a, mut counters_b| {
                    for (ch, count) in counters_b.drain() {
                        counters_a
                            .entry(ch)
                            .and_modify(|n| *n += count)
                            .or_insert(count);
                    }
                    counters_a
                })
        })
}

pub fn frequency_seq(
    input: &[&str],
    _worker_count: usize,
) -> HashMap<char, usize> {
    let mut counters = HashMap::new();
    for line in input {
        for ch in line.to_lowercase().chars().filter(|c| c.is_alphabetic()) {
            counters.entry(ch).and_modify(|n| *n += 1).or_insert(1);
        }
    }
    counters
}
