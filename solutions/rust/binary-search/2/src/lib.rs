pub fn find<T, S>(slice: S, key: T) -> Option<usize>
where
    S: AsRef<[T]>,
    T: Ord,
{
    let array = slice.as_ref();

    if array.is_empty() {
        return None;
    }

    // Borrow key for comparisons (this avoids moving out of `key` repeatedly).
    let key_ref = &key;

    // Fast-fail if key is outside the bounds.
    if key_ref < &array[0] || key_ref > &array[array.len() - 1] {
        return None;
    }

    // Use [low, high) invariants to avoid usize underflow.
    let mut low: usize = 0;
    let mut high: usize = array.len();

    while low < high {
        let mid = low + (high - low) / 2;
        use std::cmp::Ordering;
        match array[mid].cmp(key_ref) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid,
        }
    }

    None
}
