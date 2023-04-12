fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=n/2 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let n1 = 7;
    let n2 = 12;

    if is_prime(n1) {
        println!("{} is prime", n1);
    } else {
        println!("{} is not prime", n1);
    }

    if is_prime(n2) {
        println!("{} is prime", n2);
    } else {
        println!("{} is not prime", n2);
    }
}
