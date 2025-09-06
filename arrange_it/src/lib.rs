pub fn arrange_phrase(phrase: &str) -> String {

    let words: Vec<&str> = phrase.split_whitespace().collect();

    let mut result: Vec<String> = vec![String::new(); words.len()];
    
    for word in words {

        let mut position = String::new();

        let mut clean_word = String::new();
        
        for c in word.chars() {

            if c.is_ascii_digit() {

                position.push(c);
                
            } else {

                clean_word.push(c);
                
            }

        }
        
        if let Ok(index) = position.parse::<usize>() {

            if index > 0 && index <= result.len() {

                result[index - 1] = clean_word;

            }
        }
    }
    
    result.join(" ").trim().to_string()
}