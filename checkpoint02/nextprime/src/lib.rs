pub fn next_prime(nbr: u64) -> u64 {

    for i in nbr.. {
        if is_prime(i) {
            return i;
        }
    }

    return nbr;
}

fn is_prime(n: u64) -> bool {

    if n < 2 {
        return false;
    } else if n == 2 {
        return true;
    }

    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }

    return true;
}