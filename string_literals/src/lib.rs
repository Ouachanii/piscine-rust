pub fn is_empty(v: &str) -> bool {
    return v == "";
}

pub fn is_ascii(v: &str) -> bool {

    const ASCII_MAX: u8 = 127;

    for byte in v.bytes() {
        if byte > ASCII_MAX {
            return false;
        }
    }

    return true;
}

pub fn contains(v: &str, pat: &str) -> bool {
    if pat.is_empty() {
        return true;
    }

    let v_len = v.len();
    let pat_len = pat.len();

    if pat_len > v_len {
        return false;
    }

    for start in 0..=v_len - pat_len {

        let end = start + pat_len;

        if &v[start..end] == pat {
            return true;
        }

    }

    return false;
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {

    return (&v[0..index], &v[index..]);

}

pub fn find(v: &str, pat: char) -> usize {
    
    for (i, c) in v.char_indices() {
        if c == pat {
            return i;
        }
    }

    return usize::MAX;
}

