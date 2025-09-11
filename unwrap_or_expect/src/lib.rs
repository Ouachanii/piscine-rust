pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    
    if let Security::Unknown = security_level {
        return server.unwrap().to_string();
    }

    if let Security::Message = security_level {
        return server.expect("ERROR: program stops").to_string();
    }

    if let Security::Warning = security_level {
        return server.unwrap_or("WARNING: check the server").to_string();
    }

    if let Security::NotFound = security_level {
        return server.map(|url| url.to_string())
                     .unwrap_or_else(|msg| format!("Not found: {}", msg));
    }

    if let Security::UnexpectedUrl = security_level {
        return server.err()
                     .unwrap_or_else(|| panic!("{}", server.unwrap())).to_string();
    }

    todo!()
}
