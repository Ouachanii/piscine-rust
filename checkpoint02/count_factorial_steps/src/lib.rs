pub fn count_factorial_steps(factorial: u64) -> u64 {
    if factorial < 2 {
        return 0;
    }
    let mut count: u64 = 1;
    let mut fact: u64 = 1;

    loop {

        if fact >= factorial {
            break;
        }
        
        count += 1;
        fact *= count;

    }
    if fact == factorial {
        return count;
    }
    return 0;
}