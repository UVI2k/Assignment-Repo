fn merge_arrays(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut merged = Vec::with_capacity(a.len() + b.len());
    let mut i = 0;
    let mut j = 0;
    while i < a.len() && j < b.len() {
        if a[i] < b[j] {
            merged.push(a[i]);
            i += 1;
        } else {
            merged.push(b[j]);
            j += 1;
        }
    }
    merged.extend_from_slice(&a[i..]);
    merged.extend_from_slice(&b[j..]);
    merged
}

fn main() {
    let a = [1, 3, 5, 7, 9];
    let b = [2, 4, 6, 8, 10];
    let merged = merge_arrays(&a, &b);
    println!("{:?}", merged); // prints "[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]"
}
