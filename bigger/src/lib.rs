use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {

    return h.values().filter(|&&val| val > 0).max().copied().unwrap_or(0);

}
