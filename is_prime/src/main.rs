fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }

    for i in 2..num {
        if num % i == 0 {
            return false;
        }
    }

    true
}

fn main() {
    let num = 29;
    if is_prime(num) {
        println!("{} is a prime number.", num);
    } else {
        println!("{} is not a prime number.", num);
    }
}
