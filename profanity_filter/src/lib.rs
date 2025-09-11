// #[derive(Debug)]
// pub struct Message {
//     pub content: String,
//     pub author: String,
// }

// impl Message {
//     pub fn new(content: String, author: String) -> Self {
//         Self { content, author }
//     }
// }


pub fn check_ms(message: &str) -> Result<&str, &str> {
    if message.is_empty() || message.contains("stupid") {
        Err("ERROR: illegal")
    } else {
        Ok(&message)
    }
}
