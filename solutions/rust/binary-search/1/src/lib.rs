pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }
    if key < array[0] || key > array[array.len() - 1] {
        return None;
    }
    let mut low = 0;
    let mut high = array.len() - 1;
    while low <= high {
        let mid = low + (high - low) / 2;
        if array[mid] == key {
            return Some(mid);
        } else if array[mid] < key {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    None
}
