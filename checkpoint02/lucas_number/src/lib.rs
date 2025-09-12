pub fn lucas_number(n: u32) -> u32 {

    if n == 0 {
        return 2;
    } else if n == 1 {
        return 1;
    }

    let mut first_sum: u32 = 2;
    let mut sec_sum: u32 = 1;
    let mut sum: u32 = 0;

    for _ in 2..=n {
        sum = first_sum + sec_sum;
        first_sum = sec_sum;
        sec_sum = sum;
    }
    
    return sum;
}