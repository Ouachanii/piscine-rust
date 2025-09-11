#[derive(Debug)]
pub struct Message {
    pub content: String,
    pub author: String,
}

impl Message {
    pub fn new(content: String, author: String) -> Self {
        Self { content, author }
    }
}


pub fn check_ms(message: &Message) -> Result<&str, &str> {
    if message.content.is_empty() || message.content.contains("stupid") {
        Err("ERROR: illegal")
    } else {
        Ok(&message.content)
    }
}
