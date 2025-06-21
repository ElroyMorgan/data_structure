//! Sorting algorithms implementation
//! 
//! Currently provides quick sort implementation for i32 arrays.

/// Internal helper function for quick sort
/// Partitions the array and returns the pivot index
fn partition(array: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = array[low];
    let (mut left, mut right) = (low, high);

    while left < right {
        while left < right && array[right] >= pivot {
            right -= 1;
        }
        array[left] = array[right];

        while left < right && array[left] <= pivot {
            left += 1;
        }
        array[right] = array[left];
    }

    array[left] = pivot;
    left
}

/// Sorts an i32 array in-place using quick sort algorithm
///
/// # Arguments
/// * `array` - Mutable slice of i32 to be sorted
/// * `low` - Starting index of the range to sort (inclusive)
/// * `high` - Ending index of the range to sort (inclusive)
///
/// # Examples
/// ```
/// use data_structure::sorting::quick;
///
/// let mut arr = [5, 2, 4, 1, 3];
/// quick(&mut arr, 0, 4);
/// assert_eq!(arr, [1, 2, 3, 4, 5]);
/// ```
pub fn quick(array: &mut [i32], low: usize, high: usize) {
    if low < high {
        let pivot_loc = partition(array, low, high);
        quick(array, low, pivot_loc.saturating_sub(1));
        quick(array, pivot_loc + 1, high);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick() {
        let mut array = vec![5, 4, 3, 2, 1];
        let len = array.len();
        quick(&mut array, 0, len - 1);
        assert_eq!(array, vec![1, 2, 3, 4, 5]);
    }
}
