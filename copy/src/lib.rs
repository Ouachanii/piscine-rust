pub fn nbr_function(c: i32) -> (i32, f64, f64) {

    let original = c;

    let exp_value = (c as f64).exp();

    let ln_value = (c.abs() as f64).ln();

    return (original, exp_value, ln_value);

}

pub fn str_function(a: String) -> (String, String) {

    let original = a.clone();

    let mut exp_strings = Vec::new();

    for word in a.split_whitespace() {

        if let Ok(num) = word.parse::<f64>() {

            exp_strings.push(num.exp().to_string());

        }

    }

    let exp_str = exp_strings.join(" ");

    return (original, exp_str);
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {

    let original = b.clone();

    let mut ln_values = Vec::new();

    for &x in &b {

        ln_values.push((x.abs() as f64).ln());

    }

    return (original, ln_values);

}
