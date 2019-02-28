use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rayon::ThreadPoolBuilder;
use std::collections::HashMap;
use std::sync::mpsc::channel;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let (tx, rx) = channel();
    let pool = ThreadPoolBuilder::new()
        .num_threads(worker_count)
        .build()
        .unwrap();
    pool.install(|| {
        input.into_par_iter().for_each_with(tx, |sender, line| {
            let mut counters = HashMap::new();
            for ch in line.to_lowercase().chars().filter(|c| c.is_alphabetic())
            {
                counters.entry(ch).and_modify(|n| *n += 1).or_insert(1);
            }
            sender.send(counters).unwrap();
        });
    });

    let mut totals = HashMap::new();
    for mut counters in rx {
        for (ch, count) in counters.drain() {
            totals
                .entry(ch)
                .and_modify(|n| *n += count)
                .or_insert(count);
        }
    }
    totals
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
