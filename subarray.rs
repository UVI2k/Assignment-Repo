fn max_subarray_sum(a: &[i32]) -> i32 {
    let mut max_sum = a[0];
    let mut current_sum = a[0];
    for i in 1..a.len() {
        current_sum = current_sum.max(0) + a[i];
        max_sum = max_sum.max(current_sum);
    }
    max_sum
}

fn main() {
    let a = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let max_sum = max_subarray_sum(&a);
    println!("{}", max_sum); // prints "6"
}


