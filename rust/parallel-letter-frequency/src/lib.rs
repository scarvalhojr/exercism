use rayon::iter::ParallelIterator;
use rayon::slice::ParallelSlice;
use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() || worker_count == 0 {
        return HashMap::new();
    }

    // The input is split into chunks, one for each thread (worker); while this
    // ensures the number of threads will be less than worker_count, there is
    // no guarantee that the requested number of threads will actually be spawn
    // as we're relying on Rayon's global threadpool; the only way to guarantee
    // the exact number of threads would be to setup a new thread pool on each
    // call but this is expensive and would make this function much slower
    let chunk_size = (input.len() as f32 / worker_count as f32).ceil() as usize;
    input
        .par_chunks(chunk_size)
        .map(|chunk| {
            let mut counters = HashMap::new();
            for line in chunk {
                for ch in
                    line.to_lowercase().chars().filter(|c| c.is_alphabetic())
                {
                    *counters.entry(ch).or_insert(0) += 1;
                }
            }
            counters
        })
        .reduce(HashMap::new, |mut counters_a, counters_b| {
            for (ch, count) in counters_b {
                *counters_a.entry(ch).or_insert(0) += count;
            }
            counters_a
        })
}
