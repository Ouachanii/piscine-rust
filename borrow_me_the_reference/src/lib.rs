pub fn delete_and_backspace(s: &mut String) {

    let mut result = String::new();

    let mut skip = 0usize;

    for c in s.chars() {

        if c == '+' {

            skip = skip.saturating_add(1);

        } else if c == '-' {

            result.pop();

        } else {

            if skip > 0 {
                skip -= 1;
            } else {
                result.push(c);
            }

        }
    }

    *s = result;
}

pub fn do_operations(v: &mut [String]) {

    for s in v.iter_mut() {
        
        let chars = s.chars();
        let mut nb1 = 0;
        let mut nb2 = 0;
        let mut op = '+';
        let mut found_op = false;

        for c in chars {
            if c == '+' || c == '-' {
                op = c;
                found_op = true;
                break;
            } else if let Some(d) = c.to_digit(10) {
                nb1 = nb1 * 10 + d as i32;
            }
        }

        if found_op {
            let rem = &s[s.find(op).unwrap() + 1..];
            nb2 = rem.parse::<i32>().unwrap_or(0);
        }

        let result = match op {
            '+' => nb1 + nb2,
            '-' => nb1 - nb2,
            _ => 0,
        };

        *s = result.to_string();
    }
}
