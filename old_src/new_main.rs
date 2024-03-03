use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

// opens up a file with name in the argument, and returns list of lines ommiting empty lines
fn read_file(file_name: &str) -> Result<Vec<String>, io::Error> {
    let path = Path::new(file_name);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if !line.is_empty() {
            lines.push(line);
        }
    }
    Ok(lines)
}

fn clean(lines: Vec<String>) -> Vec<String> {
    lines
        .iter()
        .map(|line| {
            let comment_start = line.find("//");
            match comment_start {
                Some(start) => line[..start].to_string(),
                None => line.to_string(),
            }
        })
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect()
}

fn first_token(line: &str) -> String {
    line.split_whitespace().collect::<Vec<&str>>()[0].to_string()
}


// fn declarations(lines: Vec<String>) -> Vec<String> {
//     let var_declarations = lines
//         .iter()
//         .filter(|line| first_token(line) == "var")
//         .map(|line| line.to_string())
//         .collect();
    
//     let result = Vec::new();

//     // Todo: skończyć.
//     // for line in var_declarations {
//     //     let tokens = line.split_whitespace().collect::<Vec<&str>>();
//     //     let name = match tokens.len() {
//     //         2 => tokens[1].to_string()[1..],
//     //     }
//     //     let var_name = tokens[1].to_string();
//     //     let var_type = tokens[2].to_string();
//     //     let var_value = tokens[3].to_string();
//     //     result.push(format!("{} {} {}", var_name, var_type, var_value));
//     // }
// }

// fn instr_var(line: &str) -> String {
//     let tokens = &line
//         .split_whitespace()
//         .map(|s| s.trim().replace(",", ""))
//         .collect::<Vec<String>>()[1..];

//     match tokens.len() {
//         1 => 
//     }
// }

// mod old_main;
// use old_main::ReferenceType;
// use regex::Regex;


// fn read_ref_str(token: &str) -> Result<(String, ReferenceType), String> {
//     // Pointer and address are simple
//     if token.starts_with('@') {
//         return Ok((token[1..].to_string(), ReferenceType::Pointer));
//     } else if token.starts_with('&') {
//         return Ok((token[1..].to_string(), ReferenceType::Address));
//     }

//     // Regex to check the offset pattern
//     let offset_regex = Regex::new(r#"\$\w\w*\[\$\w\w*\]"#).unwrap();

//     // If it doesn't start with $, it's invalid at this point
//     if !token.starts_with('$') {
//         return Err("Invalid token".to_string());
//     }

//     // Check if it's an offset
//     if offset_regex.is_match(token) {
//         let offset_start = token.find('[').unwrap();
//         return Ok((token[1..offset_start].to_string(), ReferenceType::Offset(token[offset_start+1..token.len()-1])));
//     }

//     Ok(ReferenceType::Direct)
// }


fn validate(reg_str: &str, str_to_validate: &str) -> bool {
    let re = Regex::new(reg_str).unwrap();
    if let Some(captured) = re.captures(str_to_validate) {
        let first_capture = captured.get(0).unwrap(); // Can't panic cause if it were of size 0, it would be None.
        first_capture.start() == 0 && first_capture.end() == str_to_validate.len()
    }
    else {
        false
    }
}

struct Program {
    instructions: Vec<SimpleInstruction>,
    variables: Vec<Variable>,
}

impl Program {
    fn find_or_add(&mut self, var_name: &str, constant: bool) {
        match self.variables.iter().find(|v| v.name == var_name && v.constant == constant) {
            Some(var) => &var,
            None => {
                
            }
        }
    }
}

// function which given a string and program, gives a reference to the variable, or None if doesn't find it?
fn find_variable<'a>(var_ref: &str, program: &'a Program) -> Option<&'a Variable> {
    let val_num_lit = r#"\$\d[\d_]*"#;
    let val_dir_lit = r#"\$[A-Z a-z]\w*"#;
    let val_ptr_lit = r#"@[A-Z a-z]\w*"#;
    let val_off_lit = r#"\$[A-Z a-z]\w*\[\$[A-Z a-z]\w*\]"#;
    let val_adr_lit = r#"\&[A-Z a-z]\w*"#;


    if validate(val_num_lit, var_ref) {
        let num_lit_value = var_ref.replace("_", "")[1..].parse::<i32>().unwrap();
        let num_lit_name = format!("c_{}", num_lit_value);
        Program.find_or_add("num_lit_name") // Finds a variable and if can't find, adds it to the list. Returns &Variable
    }

    // ! only when we have a proper name:
    program
        .variables
        .iter()
        .find(|var| var.sig == var_sig)
}



fn istr_var(line: &str, program: &mut Program) {
    let tokens = &line
        .split_whitespace()
        .collect::<Vec<&str>>()[1..];

    match tokens.len() {
        1 => {
            let var_sig = tokens[0].to_string();
            validate("", str_to_validate)
        }
    }
}   

use regex::Regex;
fn main() {
    let file_name = "sample.txt";
    let lines = read_file(file_name).unwrap();
    let lines = clean(lines);

}