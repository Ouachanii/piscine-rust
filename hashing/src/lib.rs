use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {

    let sum: i32 = list.iter().sum();

    return sum as f64 / list.len() as f64;

}

pub fn median(list: &[i32]) -> i32 {

    let mut sorted = list.to_vec();

    sorted.sort();

    let len = sorted.len();

    if len % 2 == 1 {

        return sorted[len / 2];

    } else {

        let mid = len / 2;
        return (sorted[mid - 1] + sorted[mid]) / 2;
    }
}

pub fn mode(list: &[i32]) -> i32 {

    let mut counts = HashMap::new();

    for &num in list {
        *counts.entry(num).or_insert(0) += 1;
    }

    return counts
        .iter()
        .max_by_key(|&(_, count)| count)
        .map(|(&num, _)| num)
        .unwrap();
}
