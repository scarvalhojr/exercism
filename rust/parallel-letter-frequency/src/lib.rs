use crossbeam::scope;
use crossbeam::channel::unbounded;
use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut totals = HashMap::new();
    if input.is_empty() || worker_count == 0 {
        return totals;
    }

    let (tx, rx) = unbounded();
    let chunk_size = (input.len() as f32 / worker_count as f32).ceil() as usize;
    scope(|scope| {
        for chunk in input.chunks(chunk_size) {
            let tx = tx.clone();
            scope.spawn(move |_| {
                let mut counters = HashMap::new();
                for line in chunk {
                    line.to_lowercase()
                        .chars()
                        .filter(|chr| chr.is_alphabetic())
                        .for_each(|chr| {
                            *counters.entry(chr).or_insert(0) += 1;
                        })
                }
                tx.send(counters).unwrap();
            });
        }

        drop(tx);
        for counters in rx {
            for (chr, count) in counters {
                *totals.entry(chr).or_insert(0) += count;
            }
        }
    }).unwrap();

    totals
}
