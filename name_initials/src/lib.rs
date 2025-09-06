pub fn initials(names: Vec<&str>) -> Vec<String> {

    let mut initials = Vec::with_capacity(names.len());

    for name in names {

        let words: Vec<&str> = name.split_whitespace().collect();

        let mut init = String::new();

        for (i, word) in words.iter().enumerate() {

            if let Some(first_char) = word.chars().next() {

                init.push(first_char);

                init.push('.');

                if i < words.len() - 1 {
                    init.push(' ');
                }

            }
        }

        initials.push(init);
    }

    return initials;
}