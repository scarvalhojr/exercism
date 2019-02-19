use std::cmp::Ordering;

pub fn find<T, C>(array: C, key: T) -> Option<usize>
where
    T: Ord,
    C: AsRef<[T]>,
{
    let mut offset = 0;
    let mut sub_array = array.as_ref();

    while !sub_array.is_empty() {
        let mid_idx = sub_array.len() / 2;
        match sub_array[mid_idx].cmp(&key) {
            Ordering::Equal => {
                return Some(offset + mid_idx);
            }
            Ordering::Less => {
                sub_array = &sub_array[mid_idx + 1..];
                offset += mid_idx + 1;
            }
            Ordering::Greater => {
                sub_array = &sub_array[..mid_idx];
            }
        }
    }
    None
}
