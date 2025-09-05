pub fn factorial(num: u64) -> u64 {
    if num == 0 || num == 1 {
        return 1;
    }
    let mut fact = num;
    let mut n = num;
    loop {
        fact *= n -1;
        n -= 1;
        if  n == 1 {
            break
        }
    }
    return fact;
}
