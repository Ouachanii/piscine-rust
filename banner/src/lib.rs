use std::{collections::HashMap, num::ParseFloatError};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Flag<'a> {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: &'a str,
}

impl<'a> Flag<'a> {
    pub fn opt_flag(name: &'a str, d: &'a str) -> Self {

        let short = format!("-{}", &name.chars().next().unwrap());
        let long = format!("--{}", name);
        
        Self {
            short_hand: short,
            long_hand: long,
            desc: d,
        }

    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.long_hand, func);
        self.flags.insert(flag.short_hand, func);

    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        if let Some(func) = self.flags.get(input) {
            if argv.len() < 2 {
                return Err("Not enough arguments".to_string());
            }
            func(argv[0], argv[1]).map_err(|e| e.to_string())
        } else {
            Err("Flag not found".to_string())
        }

    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let num1: f64 = a.parse()?;
    let num2: f64 = b.parse()?;
    if num2 == 0.0 {
        return Ok("Error: Division by zero".to_string());
    }
    Ok((num1 / num2).to_string())

}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let num1: f64 = a.parse()?;
    let num2: f64 = b.parse()?;
    if num2 == 0.0 {
        return Ok("Error: Division by zero".to_string());
    }
    Ok((num1 % num2).to_string())
}
