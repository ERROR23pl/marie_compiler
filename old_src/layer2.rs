#[allow(unused_variables, dead_code)]

use regex::Regex;

use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

// opens up a file with name in the argument, and returns list of lines ommiting empty lines
pub fn read_file(file_name: &str) -> Result<Vec<String>, io::Error> {
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

enum NativeInstruction {
    Add, Subt, Addi, Clear, Store, Load, Input, Output, Jump, Skipcond,Jns, Jumpi, Storei, Loadi, Halt
}

#[derive(Debug)]
enum Layer1Instruction {
    Add, Subt, Clear, Store, Load, Input, Output, Jump, Skipcond, Jns, Halt
}

// Todo: Implement from_str like it's supposed to be.
impl Layer1Instruction {
    fn from_str(s: &str) -> Option<Layer1Instruction> {
        match s {
            "add" => Some(Layer1Instruction::Add),
            "subt" => Some(Layer1Instruction::Subt),
            "clear" => Some(Layer1Instruction::Clear),
            "store" => Some(Layer1Instruction::Store),
            "load" => Some(Layer1Instruction::Load),
            "input" => Some(Layer1Instruction::Input),
            "output" => Some(Layer1Instruction::Output),
            "jump" => Some(Layer1Instruction::Jump),
            "skipcond" => Some(Layer1Instruction::Skipcond),
            "jns" => Some(Layer1Instruction::Jns),
            "halt" => Some(Layer1Instruction::Halt),
            _ => None
        }
    }
    fn has_parameter(&self) -> bool {
        matches!(self,
              Layer1Instruction::Add 
            | Layer1Instruction::Subt 
            | Layer1Instruction::Store
            | Layer1Instruction::Load
            | Layer1Instruction::Jump
            | Layer1Instruction::Skipcond
            | Layer1Instruction::Jns
        
        )
    }
}

impl NativeInstruction {
    fn from_str(s: &str) -> Option<NativeInstruction> {
        match s {
            "add" => Some(NativeInstruction::Add),
            "subt" => Some(NativeInstruction::Subt),
            "addi" => Some(NativeInstruction::Addi),
            "clear" => Some(NativeInstruction::Clear),
            "store" => Some(NativeInstruction::Store),
            "load" => Some(NativeInstruction::Load),
            "input" => Some(NativeInstruction::Input),
            "output" => Some(NativeInstruction::Output),
            "jump" => Some(NativeInstruction::Jump),
            "skipcond" => Some(NativeInstruction::Skipcond),
            "jns" => Some(NativeInstruction::Jns),
            "jumpi" => Some(NativeInstruction::Jumpi),
            "storei" => Some(NativeInstruction::Storei),
            "loadi" => Some(NativeInstruction::Loadi),
            "halt" => Some(NativeInstruction::Halt),
            _ => None
        }
    }

}

struct Program {
    name: String,
    instructions: Vec<SimpleInstruction>,
    variables: Vec<Variable>,
}


fn split_string(s: &str) -> Vec<String> {
    s.split_whitespace().map(|s| s.to_string()).collect()

}

fn find_instruction(tokens: Vec<String>) -> Option<Layer1Instruction> {
    let first_string = tokens.get(0)?;
    Layer1Instruction::from_str(first_string)
}




fn determine_signature(token: &str) -> Result<VarSignature, String> {
    // Pointer and address are simple
    if token.starts_with('@') {
        return Ok(VarSignature::Pointer);
    } else if token.starts_with('&') {
        return Ok(VarSignature::Address);
    }

    // Regex to check the offset pattern
    let offset_regex = Regex::new(r#"\$\w\w*\[\$\w\w*\]"#).unwrap();

    // If it doesn't start with $, it's invalid at this point
    if !token.starts_with('$') {
        return Err("Invalid token".to_string());
    }

    // Check if it's an offset
    if offset_regex.is_match(token) {
        let offset_start = token.find('[').unwrap();
        return Ok(VarSignature::Offset(token[offset_start+1..token.len()-1].to_string()));
    }

    Ok(VarSignature::Direct)
}

#[derive(Debug)]
enum VarSignature {
    Direct,
    Pointer,
    Offset(String),
    Address
}

#[derive(Debug)]
struct SimpleInstruction {
    instruction: Layer1Instruction,
    parameter_name: String,
    parameter_signature: VarSignature,
    label: Option<String>
}

impl SimpleInstruction {
    fn from_strings(l1instr: Layer1Instruction, variable: &str) -> Result<SimpleInstruction, String> {
        let signature = determine_signature(variable)?;

        let name = match signature {
            VarSignature::Offset(_) => variable.split('[').collect::<Vec<&str>>()[0][1..].to_string(), // Remove the $ sign and [...] syntax.
            _ => variable.to_string()[1..].to_string()
        };
        
        Ok(SimpleInstruction {
            instruction: l1instr,
            parameter_name: name,
            parameter_signature: signature,
            label: None
        })
    }
}

#[derive(Debug)]
struct Variable {
    name: String,
    constant: bool,
    address: u32,
}

fn remove_comments(s: &str) -> String {
    let comment_start = s.find("//");
    match comment_start {
        Some(index) => s[..index].to_string(),
        None => s.to_string()
    }
}


// Todo: ERROR HANDLING
fn add_instr(instr: &str) -> Vec<SimpleInstruction> {
    let tokens: Vec<String> = instr
        .split_whitespace()
        .map(|s| s.replace(",", ""))
        .collect();

    use SimpleInstruction as SI;
    use Layer1Instruction as L1I;
    match tokens.len() {
        2 => vec![SI::from_strings(L1I::Add, &tokens[1]).unwrap()],
        3 => vec![
            SI::from_strings(L1I::Load, &tokens[1]).unwrap(),
            SI::from_strings(L1I::Add, &tokens[2]).unwrap(),
            SI::from_strings(L1I::Store, &tokens[1]).unwrap()
        ],
        4 => vec![
            SI::from_strings(L1I::Load, &tokens[2]).unwrap(),
            SI::from_strings(L1I::Add, &tokens[3]).unwrap(),
            SI::from_strings(L1I::Store, &tokens[1]).unwrap()
        ],
        _ => vec![]
    }
}