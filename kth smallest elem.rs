fn find_kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > arr.len() {
        return None;
    }
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();
    Some(sorted_arr[k - 1])
}

fn main() {
    let arr = [1, 5, 3, 8, 2, 7];
    let k = 3;
    if let Some(kth_smallest) = find_kth_smallest(&arr, k) {
        println!("The {}th smallest element in {:?} is {}", k, arr, kth_smallest);
    } else {
        println!("No element found for k = {} in {:?}", k, arr);
    }
}
