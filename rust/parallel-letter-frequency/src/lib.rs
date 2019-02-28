use rayon::iter::ParallelIterator;
use rayon::slice::ParallelSlice;
use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() || worker_count == 0 {
        return HashMap::new();
    }

    let chunk_size = (input.len() as f32 / worker_count as f32).ceil() as usize;
    input
        .par_chunks(chunk_size)
        .map(|chunk| {
            let mut counters = HashMap::new();
            for line in chunk {
                for ch in
                    line.to_lowercase().chars().filter(|c| c.is_alphabetic())
                {
                    (*counters.entry(ch).or_insert(0)) += 1;
                }
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
}
