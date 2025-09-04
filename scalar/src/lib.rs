pub fn sum(left: u8, right: u8) -> u8 {
    left + right
}


pub fn diff(x: i16, y: i16) -> i16 {
    x - y
}

pub fn pro(x: i8, y: i8) -> i16 {
    (x as i16) * (y as i16)
}

pub fn quo(x: i32, y: i32) -> i32 {
    if y != 0 {
        return x/y;
    }
    return 0;
}

pub fn rem(x: i32, y: i32) -> i32 {
    if y != 0 {
        return x % y;
    }
    return 0;
}