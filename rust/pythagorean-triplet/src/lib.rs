pub fn find() -> Option<u32> {
    let sum_target = 1_000;

    for a in 1..(sum_target / 3) {
        for b in (a + 1).. {
            let c = sum_target - a - b;
            if c <= b { break; }
            if a * a + b * b == c * c {
                return Some(a * b * c);
            }
        }
    }
    None
}
