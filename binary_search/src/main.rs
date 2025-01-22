use std::cmp::Ordering;

/// Performs a binary search on a sorted slice `slice` for the value `key`.
/// Returns `Some(index)` if the key is found, otherwise `None`.
fn binary_search<T: Ord>(slice: &[T], key: &T) -> Option<usize> {
    let mut left = 0;
    let mut right = slice.len();

    while left < right {
        // Middle index; avoid potential overflow by using left + (right - left) / 2
        let mid = left + (right - left) / 2;

        match key.cmp(&slice[mid]) {
            Ordering::Less => {
                // If the kehy is less, we look in the left half
                right = mid;
            }
            Ordering::Greater => {
                // If the key is greater, we look in the right half
                left = mid + 1;
            }
            Ordering::Equal => {
                // Key found at position mid
                return Some(mid);
            }
        }
    }

    None
}

fn main() {
    let sorted_numbers = [1, 3, 5, 7, 9, 11, 13, 15];
    let key = 7;

    // Seawrch for 7 in the array
    match binary_search(&sorted_numbers, &key) {
        Some(index) => println!("Found {} at index {}.", key, index),
        None => println!("{} not found in the slice.", key),
    }

    // Search for a value not in the array
    let missing = 4;
    match binary_search(&sorted_numbers, &missing) {
        Some(index) => println!("Found {} at index {}.", missing, index),
        None => println!("{} not found in the slice.", missing),
    }
}
