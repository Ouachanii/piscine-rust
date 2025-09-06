pub fn first_subword(mut s: String) -> String {

    let mut first_subword = String::new();

    let mut chars = s.chars();

    while let Some(c) = chars.next() {

        if c == '_' {
            break;
        }

        if !first_subword.is_empty() && c.is_uppercase() {
            break;
        }

        first_subword.push(c);
    }

    return first_subword;
}
