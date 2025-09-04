pub fn sum(left: u8, right: u8) -> u8 {
    left.checked_add(right).unwrap()
}

pub fn diff(x: i16, y: i16) -> i16 {
    x.checked_sub(y).unwrap()
}

pub fn pro(x: i8, y: i8) -> i8 {
    x.checked_mul(y).unwrap()
}

pub fn quo(x: f32, y: f32) -> f32 {
    if y != 0.0 {
        x / y
    } else {
        0.0
    }
}

pub fn rem(x: f32, y: f32) -> f32 {
    if y != 0.0 {
        x % y
    } else {
        0.0
    }
}
