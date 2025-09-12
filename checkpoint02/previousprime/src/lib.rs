pub fn prev_prime(nbr: u64) -> u64  {

    if nbr < 3 {
        return 0;
    }

    for i in (2..nbr).rev() {

        if is_prime(i) {
            return i;
        }

    }

    return 0;
}

fn is_prime(n: u64) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}