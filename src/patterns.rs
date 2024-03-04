use std::{num::ParseIntError};

use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum SinglePattern {
    Numeral,
    Direct,
    Pointer,
    Address,
}

impl SinglePattern {
    pub fn from_refr(refr: &str) -> Option<SinglePattern> {
        match refr.chars().nth(0) {
            None => None,
            Some('@') => Some(SinglePattern::Pointer),
            Some('&') => Some(SinglePattern::Address),
            Some('$') => {
                if let Some(c) = refr.chars().nth(1) {
                    if c.is_digit(10) {
                        Some(SinglePattern::Numeral)
                    }
                    else if c.is_alphabetic() {
                        Some(SinglePattern::Direct)
                    }
                    else {
                        None
                    }
                }
                else {
                    None
                }
            }
            _ => None
        
        }
    }

    pub fn refr_name(refr: &str) -> Option<String> {
        match SinglePattern::from_refr(refr) {
            None => None,
            Some(SinglePattern::Numeral) => Some(format!(":{}", refr[1..].replace("_", ""))),
            Some(SinglePattern::Direct) | Some(SinglePattern::Pointer) => Some(refr[1..].to_string()),
            Some(SinglePattern::Address) => Some(format!("{}:addr", refr[1..].to_string()))
        }
    }

    pub fn all_patterns() -> Vec<SinglePattern> {
        vec![
            SinglePattern::Numeral,
            SinglePattern::Direct,
            SinglePattern::Pointer,
            SinglePattern::Address
        ]
    }
}

pub trait Pattern {
    fn get_pattern(&self) -> String;
    fn get_regex(&self) -> Regex;
    fn validate(&self, str_to_validate: &str) -> bool {
        let re = self.get_regex();
        if let Some(captured) = re.find(str_to_validate) { // Can't panic cause if it were of size 0, it would be None.
            captured.start() == 0 && captured.end() == str_to_validate.len()
        }
        else {
            false
        }
    }
}

impl Pattern for SinglePattern{
    fn get_pattern(&self) -> String {
        match self {
            SinglePattern::Numeral => r#"\$\d[\d_]*"#.to_string(),
            SinglePattern::Direct => r#"\$[A-Za-z]\w*"#.to_string(),
            SinglePattern::Pointer => r#"@[A-Za-z]\w*"#.to_string(),
            SinglePattern::Address => r#"\&[A-Za-z]\w*"#.to_string()
        }
    }

    fn get_regex(&self) -> Regex {
        Regex::new(&self.get_pattern()).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FullPatern {
    Numeral,
    Direct,
    Pointer,
    Address,
    OffsetNum,
    OffsetVar,
}

pub fn num_lit_to_i16(num_lit: &str) -> Result<i16, ParseIntError>  {    
    num_lit[1..].replace("_", "").parse()
}

impl Pattern for FullPatern{
    fn get_pattern(&self) -> String {
        match self {
            FullPatern::Numeral => r#"\$\d[\d_]*"#.to_string(),
            FullPatern::Direct => r#"\$[A-Za-z]\w*"#.to_string(),
            FullPatern::Pointer => r#"@[A-Za-z]\w*"#.to_string(),
            FullPatern::Address => r#"\&[A-Za-z]\w*"#.to_string(),
            FullPatern::OffsetNum => r#"@[A-Za-z]\w*\[\$\d[\d_]*\]"#.to_string(),
            FullPatern::OffsetVar => r#"@[A-Za-z]\w*\[\$[A-Z a-z]\w*\]"#.to_string(),
        }
    }

    fn get_regex(&self) -> Regex {
        Regex::new(&self.get_pattern()).unwrap()
    }
}


pub fn validate(reg_str: &str, str_to_validate: &str) -> bool {
    let re = Regex::new(reg_str).unwrap();
    if let Some(captured) = re.captures(str_to_validate) {
        let first_capture = captured.get(0).unwrap(); // Can't panic cause if it were of size 0, it would be None.
        first_capture.start() == 0 && first_capture.end() == str_to_validate.len()
    }
    else {
        false
    }
}