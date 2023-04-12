fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }

    for i in 2..=n / 2 {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn main() {
    let n = 17;
    if is_prime(n) {
        println!("{} is prime", n);
    } else {
        println!("{} is not prime", n);
    }
}
