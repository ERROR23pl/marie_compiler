#[allow(unused_variables, dead_code)]

use std::collections::HashMap;
use regex::Regex;


#[derive(Debug, Clone)]
struct Variable {
    name: String,
    default_value: i16,
    constant: bool,
    address: u16,
}

impl Variable {
    fn to_native(&self) -> String {
        format!("{},\tdec {}", self.name, self.default_value)
    }
}

#[derive(Debug, Clone)]
struct VariableGenerator {
    address: u16,
}

impl VariableGenerator {
    fn new(start_address: u16) -> VariableGenerator {
        VariableGenerator {
            address: start_address,
        }
    }

    fn generate(&mut self, name: &str, default_value: i16, constant: bool) -> Variable {
        let var = Variable {
            name: name.to_string(),
            default_value,
            constant,
            address: self.address,
        };

        self.address += 1;
        var
    }
}

impl Default for VariableGenerator {
    fn default() -> Self {
        VariableGenerator { address: 1 }
    }
}

#[derive(Debug, Clone)]
pub enum ReferenceType<'a> {
    Direct,
    Pointer,
    Offset(&'a Variable),
    Address,
}

#[derive(Debug, Clone)]
struct Reference<'a> {
    var: &'a Variable,
    reference_type: ReferenceType<'a>,
}

impl Reference<'_> {
    fn new<'a>(var: &'a Variable, reference_type: ReferenceType<'a>) -> Reference<'a> {
        Reference {
            var: &var,
            reference_type,
        }
    }
    fn to_string(&self) -> String {
        match self.reference_type {
            ReferenceType::Direct => format!("${}", self.var.name),
            ReferenceType::Pointer => format!("@{}", self.var.name),
            ReferenceType::Address => format!("&{}", self.var.name),
            ReferenceType::Offset(off) => format!("${}[${}]", self.var.name, off.name),
        }
    }
}

#[derive(Debug, Clone)]
enum SkipcondType {
    GreaterThanZero,
    LessThanZero,
    Zero,
}

impl SkipcondType {
    fn to_string(&self) -> String {
        match self {
            SkipcondType::GreaterThanZero => "800",
            SkipcondType::LessThanZero => "000",
            SkipcondType::Zero => "400",
        }.to_string()
    }
}

type Label = String;

#[derive(Debug, Clone)]
enum SimpleInstruction<'a> {
    Add(Reference<'a>), Subt(Reference<'a>), Store(Reference<'a>), Load(Reference<'a>), Jns(Reference<'a>),
    Skipcond(SkipcondType),
    Jump(Label),
    Clear, Input, Output, Halt,
}

impl SimpleInstruction<'_> {
    fn name(&self) -> String {
        use SimpleInstruction::*;
        match self {
            Add(_) => "add",
            Subt(_) => "subt",
            Store(_) => "store",
            Load(_) => "load",
            Jns(_) => "jns",
            Skipcond(_) => "skipcond",
            Jump(_) => "jump",
            Clear => "clear",
            Input => "input",
            Output => "output",
            Halt => "halt",
        }.to_string()
    }

    fn to_native(&self) -> String {
        use SimpleInstruction as SI;
        match self {
            SI::Add(reference) | SI::Store(reference) | SI::Load(reference) | SI::Jns(reference) => {
                match reference.reference_type {
                    ReferenceType::Direct => format!("{} {}", self.name(), reference.var.name),
                    ReferenceType::Pointer => format!("{}i {}", self.name(), reference.var.name),
                    ReferenceType::Address => format!("{} {}_addr", self.name(), reference.var.name), // Todo: add _addr variable to variable list.
                    ReferenceType::Offset(off) => {
                        format!("store temp_acc\nload {}\nadd {}\nstore temp_addr\n{}i temp_addr", reference.var.name, off.name, self.name())
                    }
                }
            },

            SI:: Subt(reference) => {
                match reference.reference_type {
                    ReferenceType::Direct => format!("subt {}", reference.var.name),
                    ReferenceType::Pointer => {
                        format!("store temp_acc\nloadi {}\njns subti", reference.var.name)
                     },
                    ReferenceType::Address => format!("subt {}_addr", reference.var.name), // Todo: add _addr variable to variable list.
                    ReferenceType::Offset(off) => {
                        format!("store temp_acc\nload {}\nadd {}\nstore temp_addr\njns subti", reference.var.name, off.name)
                    }
                }
            }

            SI::Skipcond(cond) => format!("{} {}", self.name(), cond.to_string()),
            SI::Jump(label) => format!("{} {}", self.name(), label),
            SI::Clear | SI::Input | SI::Output | SI::Halt => self.name()
        }
    }
}

#[derive(Default, Clone)]
struct Program<'a> {
    instructions: Vec<SimpleInstruction<'a>>,
    variables: Vec<Variable>,
    var_gen: VariableGenerator,
}

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

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


impl<'a> Program<'a> {
    fn add_instruction(&mut self, instruction: SimpleInstruction<'a>) {
        self.instructions.push(instruction);
    }

    fn add_variable(&mut self, name: &str, default_value: i16, constant: bool) {
        self.variables.push(
            self.var_gen.generate(name, default_value, constant)
        );
    }

    fn get_variable(&self, name: &str) -> Option<&Variable> {
        self.variables.iter().find(|v| v.name == name)
    }

    fn get_reference(&self, refr: &str) -> Option<Reference> {
        let val_num_lit = r#"\$\d[\d_]*"#;
        let val_dir_lit = r#"\$[A-Za-z]\w*"#;
        let val_ptr_lit = r#"@[A-Za-z]\w*"#;
        let val_adr_lit = r#"\&[A-Za-z]\w*"#;
        let val_off_lit = r#"@[A-Za-z]\w*\[\$[A-Z a-z]\w*\]"#;
        let val_off_num_lit = r#"@[A-Za-z]\w*\[\$\d[\d_]*\]"#;

        if validate(val_num_lit, refr) {
            if let Some(var) = self.get_variable(&format!("c_{}", &refr[1..])) {
                Some(Reference::new(var, ReferenceType::Direct))
            }
            else {
                None
            }
        }
        else if validate(val_dir_lit, refr) {
            if let Some(var) = self.get_variable(&refr[1..]) {
                Some(Reference::new(var, ReferenceType::Direct))
            }
            else {
                None
            }
        }
        else if validate(val_ptr_lit, refr) {
            if let Some(var) = self.get_variable(&refr[1..]) {
                Some(Reference::new(var, ReferenceType::Pointer))
            }
            else {
                None
            }
        }
        else if validate(val_adr_lit, refr) {
            if let Some(var) = self.get_variable(&format!("{}_addr", &refr[1..])) {
                Some(Reference::new(var, ReferenceType::Pointer))
            }
            else {
                None
            }
        }
        else if validate(val_off_lit, refr) {
            let var_re = Regex::new(val_dir_lit).unwrap();
            let ptr_re = Regex::new(val_ptr_lit).unwrap();

            let pointer = ptr_re.find(refr).unwrap().as_str();
            let offset = var_re.find(refr).unwrap().as_str();

            let pointer_var = self.get_variable(&pointer[1..]);
            let offset_var = self.get_variable(&offset[1..]);

            if let (Some(p), Some(o)) = (pointer_var, offset_var) {
                Some(Reference::new(p, ReferenceType::Offset(o)))
            }
            else {
                None
            }
        }
        else if validate(val_off_num_lit, refr) {
            let num_re = Regex::new(val_num_lit).unwrap();
            let ptr_re = Regex::new(val_ptr_lit).unwrap();

            let pointer = ptr_re.find(refr).unwrap().as_str();
            let offset = num_re.find(refr).unwrap().as_str();

            let pointer_var = self.get_variable(&pointer[1..]);
            let offset_var = self.get_variable(&format!("c_{}", &offset[1..].replace("_", "")));

            if let (Some(p), Some(o)) = (pointer_var, offset_var) {
                Some(Reference::new(p, ReferenceType::Offset(o)))
            }
            else {
                None
            }
        }
        else {
            None
        }
    }


    fn to_string(&self) -> String {
        let result = self.variables
            .iter()
            .map(|v| v.to_native())
            .collect::<Vec<String>>()
            .join("\n");

        result + "\n" + &self.instructions
            .iter()
            .map(|i| i.to_native())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

fn num_literals(lines: &[String]) -> Vec<i16> {
    let num_lit_re = Regex::new(r#"\$\d[\d_]*"#).unwrap();
    
    let mut result: Vec<i16> = lines
        .iter()
        .map(|l| num_lit_re.find_iter(l))
        .flatten()
        .map(|s| s.as_str().replace("$", "").replace("_", "").parse::<i16>())
        .flatten()
        .collect();

    result.sort();
    result.dedup();
    result
}

fn var_declarations(lines: &[String]) -> Vec<(String, i16)> {
    // Find all variable declarations
    let var_declarations = lines
        .iter()
        .filter(|l| l.split_whitespace().next() == Some("var"));

    let var_re = Regex::new(r#"\$[A-za-z][\w-]*"#).unwrap();
    let num_lit_re = Regex::new(r#"\$\d[\d_]*"#).unwrap();


    // Find all variables without default values
    let no_default_re = r#"var \$[A-za-z][\w-]*"#;
    let no_default = var_declarations
        .clone()
        .filter(|s| validate(no_default_re, s.trim()))
        .map(|s| (var_re.find(s).unwrap().as_str()[1..].to_string(), 0));

    // Find all variables with default values
    let default_re = r#"var \$[A-za-z][\w-]* *= *\$\d[\d_]*"#;
    let default = var_declarations
        .clone()
        .filter(|s| validate(default_re, s.trim()))
        .map(|s| {
            let declared_var = var_re.find(s).unwrap().as_str()[1..].to_string();
            let default_value = num_lit_re.find(s).unwrap().as_str()[1..].replace("_", "").parse::<i16>().unwrap();
            (declared_var, default_value)
        });

    no_default.chain(default).collect()
}

fn const_declarations(lines: &[String]) -> Vec<(String, i16)> {
    // Find all variable declarations
    let const_declarations = lines
        .iter()
        .filter(|l| l.split_whitespace().next() == Some("const"));

    let var_re = Regex::new(r#"\$[A-za-z][\w-]*"#).unwrap();
    let num_lit_re = Regex::new(r#"\$\d[\d_]*"#).unwrap();

    // Find all variables with default values
    let default_re = r#"const \$[A-za-z][\w-]* *= *\$\d[\d_]*"#;
    let default = const_declarations
        .filter(|s| validate(default_re, s.trim()))
        .map(|s| {
            let declared_var = var_re.find(s).unwrap().as_str()[1..].to_string();
            let default_value = num_lit_re.find(s).unwrap().as_str()[1..].replace("_", "").parse::<i16>().unwrap();
            (declared_var, default_value)
        });

    default.collect()
}

fn addresses(lines: &[String]) -> Vec<String> {
    let addr_re = Regex::new(r#"&[A-za-z][\w-]*"#).unwrap();
    lines
        .iter()
        .map(|l| addr_re.find_iter(l))
        .flatten()
        .map(|s| s.as_str()[1..].to_string())
        .collect()
}

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

// Todo add array initialization
// Todo add multiple variable declaration in one line
fn var_handler<'a>(line: &'a str, prog: &'a Program<'a>) -> Vec<SimpleInstruction<'a>> {
    let no_default_re = r#"var \$[A-za-z][\w-]*"#;
    let default_re = r#"var \$[A-za-z][\w-]* *= *\$\d[\d_]*"#;
    // let array_literal_re = r#"var \$[A-za-z][\w-]* *= \[( *\$\d[\d_]* *,)* *\$\d[\d_]* *\]"#; // var $arr = [$1, $2, $3]
    // let array_size_re = r#"var \$[A-Za-z]\w*\[\$\d[\d_]*\]"#;

    if validate(no_default_re, line.trim()) {
        Vec::new()
    }
    else if validate(default_re, line.trim()) {
        let var_re = Regex::new(r#"\$[A-za-z][\w-]*"#).unwrap();
        let num_lit_re = Regex::new(r#"\$\d[\d_]*"#).unwrap();

        let variable = prog.get_reference(var_re.find(line).unwrap().as_str());
        let value = prog.get_reference(num_lit_re.find(line).unwrap().as_str());
        
        vec![
            SimpleInstruction::Load(value.unwrap()),
            SimpleInstruction::Store(variable.unwrap())
        ]
    }
    else {
        Vec::new()
    }

}

fn store_handler<'a>(line: &'a str, prog: &'a Program<'a>) -> Vec<SimpleInstruction<'a>> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    
    let refr = prog.get_reference(tokens[1]).unwrap();

    vec![
        SimpleInstruction::Store(refr)
    ]
}

fn load_handler<'a>(line: &'a str, prog: &'a Program<'a>) -> Vec<SimpleInstruction<'a>> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    
    let refr = prog.get_reference(tokens[1]).unwrap();

    vec![
        SimpleInstruction::Load(refr)
    ]
}

fn clear_handler<'a>(line: &'a str, prog: &'a Program<'a>) -> Vec<SimpleInstruction<'a>> {
    vec![
        SimpleInstruction::Clear
    ]
}

fn input_handler<'a>(line: &'a str, prog: &'a Program<'a>) -> Vec<SimpleInstruction<'a>> {
    vec![
        SimpleInstruction::Input
    ]
}

fn output_handler<'a>(line: &'a str, prog: &'a Program<'a>) -> Vec<SimpleInstruction<'a>> {
    vec![
        SimpleInstruction::Output
    ]
}

fn halt_handler<'a>(line: &'a str, prog: &'a Program<'a>) -> Vec<SimpleInstruction<'a>> {
    vec![
        SimpleInstruction::Halt
    ]
}

fn add_handler<'a>(line: &'a str, prog: &'a Program<'a>) -> Vec<SimpleInstruction<'a>> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    
    match tokens.len() {
        2 => {
            let refr = prog.get_reference(tokens[1]).unwrap();
            vec![
                SimpleInstruction::Add(refr)
            ]
        },
        3 => {
            let dest = prog.get_reference(tokens[1]).unwrap();
            let src = prog.get_reference(tokens[2]).unwrap();
            vec![
                SimpleInstruction::Load(src),
                SimpleInstruction::Add(dest.clone()),
                SimpleInstruction::Store(dest)
            ]
        }
        4.. => {
            let dest = prog.get_reference(tokens[1]).unwrap();
            let src = prog.get_reference(tokens[2]).unwrap();
            let src2 = prog.get_reference(tokens[3]).unwrap();
            vec![
                SimpleInstruction::Load(src),
                SimpleInstruction::Add(src2),
                SimpleInstruction::Store(dest)
            ]
        }
        _ => Vec::new()
    }
}

fn subt_handler<'a>(line: &'a str, prog: &'a Program<'a>) -> Vec<SimpleInstruction<'a>> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    
    match tokens.len() {
        2 => {
            let refr = prog.get_reference(tokens[1]).unwrap();
            vec![
                SimpleInstruction::Subt(refr)
            ]
        },
        3 => {
            let dest = prog.get_reference(tokens[1]).unwrap();
            let src = prog.get_reference(tokens[2]).unwrap();
            vec![
                SimpleInstruction::Load(src),
                SimpleInstruction::Subt(dest.clone()),
                SimpleInstruction::Store(dest)
            ]
        }
        4.. => {
            let dest = prog.get_reference(tokens[1]).unwrap();
            let src = prog.get_reference(tokens[2]).unwrap();
            let src2 = prog.get_reference(tokens[3]).unwrap();
            vec![
                SimpleInstruction::Load(src),
                SimpleInstruction::Subt(src2),
                SimpleInstruction::Store(dest)
            ]
        }
        _ => Vec::new()
    }
}




fn main() {
    let file = read_file("../sample.txt").unwrap();
    let mut prog = Program::default();
        
    for var in var_declarations(&file).iter() {
        prog.add_variable(&var.0, var.1, false)
    }

    for constant in const_declarations(&file).iter() {
        prog.add_variable(&constant.0, constant.1, true)
    }
    
    for num_lit in num_literals(&file) {
        let name = "c_".to_string() + &num_lit.to_string();
        prog.add_variable(&name, num_lit, true)
    }

    for address in addresses(&file).iter() {
        let name = address.to_string() + &"_addr".to_string();
        if let Some(var) = prog.get_variable(address) {
            prog.add_variable(&name, var.address as i16, true)
        }
    }

    let mut handlers: HashMap<String, Box<dyn for<'a> Fn(&'a str, &'a Program<'a>) -> Vec<SimpleInstruction<'a>>>> = HashMap::new();

    handlers.insert("var".to_string(), Box::new(var_handler));
    handlers.insert("store".to_string(), Box::new(store_handler));
    handlers.insert("load".to_string(), Box::new(load_handler));
    handlers.insert("clear".to_string(), Box::new(clear_handler));
    handlers.insert("input".to_string(), Box::new(input_handler));
    handlers.insert("output".to_string(), Box::new(output_handler));
    handlers.insert("halt".to_string(), Box::new(halt_handler));
    handlers.insert("add".to_string(), Box::new(add_handler));
    handlers.insert("subt".to_string(), Box::new(subt_handler));

    prog.add_instruction(SimpleInstruction::Jump("main".to_string()));

    let variables_copy = prog.clone();
    for line in file.iter() {
        if let Some(first_token) = line.split_whitespace().next() {
            if let Some(handler) = handlers.get(first_token) {
                let instructions = handler(line, &variables_copy); // Clone the prog variable
                for instruction in instructions {
                    prog.add_instruction(instruction);
                }
            }
        }
    }

    // std::mem::drop(variables_copy);

    println!("{}", prog.to_string());
}