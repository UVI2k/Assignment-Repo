fn find_median(arr: &[i32]) -> Option<f32> {
    let len = arr.len();
    if len == 0 {
        return None;
    }
    let middle = len / 2;
    if len % 2 == 0 {
        // Even length array
        Some((arr[middle - 1] as f32 + arr[middle] as f32) / 2.0)
    } else {
        // Odd length array
        Some(arr[middle] as f32)
    }
}

fn main() {
    let arr = [1, 2, 3, 4, 5, 6];
    if let Some(median) = find_median(&arr) {
        println!("The median of {:?} is {}", arr, median);
    } else {
        println!("No median found for {:?}", arr);
    }
}
