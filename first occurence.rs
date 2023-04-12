fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;

        if arr[mid] < target {
            low = mid + 1;
        } else if arr[mid] > target {
            high = mid - 1;
        } else {
            // found the target, now find the first occurrence
            let mut i = mid;
            while i > 0 && arr[i - 1] == target {
                i -= 1;
            }
            return Some(i);
        }
    }

    None // target not found
}

fn main() {
    let arr = [1, 2, 2, 3, 4, 4, 4, 5];
    let target = 4;
    if let Some(index) = find_first_occurrence(&arr, target) {
        println!("Found {} at index {}", target, index);
    } else {
        println!("{} not found", target);
    }
}
