use crate::patterns;
use crate::patterns::validate;
use crate::patterns::Pattern;
use crate::schema::Variable;
use crate::schema::Line;
use regex::Regex;

use crate::patterns::SinglePattern;


// Takes a line and finds what variables are used.
pub fn fetch_vars(line: &Line) -> Vec<String> {
    let mut should_exist = Vec::new();

    // function refr_name() returns a name of refered variable.
    // `&var` refers to `var:addr` so it's handled separately.
    let patterns = vec![
        SinglePattern::Numeral,
        SinglePattern::Direct,
        SinglePattern::Pointer,
    ];

    // Find every variable name, add to the list.
    for pattern in patterns.iter() {
        let regex = pattern.get_regex();
        for cap in regex.find_iter(&line.content) {
            should_exist.push(SinglePattern::refr_name(&cap.as_str()).expect("This should be impossible!"));
        }
    }
    
    // Addresses are added automatically without declarations, so we don't have
    // to check if they exist, however the variables who's addresses are used
    // have to be checked for existence.
    for cap in SinglePattern::Address.get_regex().find_iter(&line.content) {
        should_exist.push(cap.as_str()[1..].to_string());
    }

    should_exist
}

pub fn let_handler(line: &Line, already_declared: &[String]) -> (String, i16, bool) {
    let validate_re = r#"let +(const +)? *(\$[A-Za-z]\w*)( *= *(\$\d[\d_]*|\$[A-Za-z]\w*))?"#;

    if !validate(validate_re, &line.content) {
        panic!("\nInvalid variable declaration on line {}!\nExpected `let [const] <var_name> [= <reference>]\nFound: `{}`\n", line.num, line.content);
    }

    let tokens = line.content.split_whitespace().collect::<Vec<&str>>();
    match tokens.len() {
        2 => {
            // Variable declaration without initialization.
            let validate_re = r#"let +(\$[A-Za-z]\w*)"#;
            if !validate(validate_re, &line.content) {
                panic!("Invalid variable declaration on line {}!", line.num);
            }
            let var_name = SinglePattern::refr_name(tokens[1]).expect("Imppossible");
            if already_declared.contains(&var_name) {
                panic!("Variable {} on line {} is already declared!", var_name, line.num);
            }
            else {
                (var_name, 0, false)
            }

        }
        // `let $var = $10` or `let $var = $other`
        4 => {
            // e.g. let $var = $10
            let validate_num_re = r#"let +(\$[A-Za-z]\w*)( *= *\$\d[\d_]*)"#;
            
            // check if the variable is already declared.
            let var_name = SinglePattern::refr_name(tokens[1]).unwrap();
            if already_declared.contains(&var_name) {
                panic!("Variable {} on line {} is already declared!", var_name, line.num);
            }

            // if numeral literal was present, use it as default value.
            let mut default_value: i16 = 0;
            if validate(validate_num_re, &line.content) {
                default_value = patterns::num_lit_to_i16(tokens[3]).unwrap();
            }
            
            (var_name, default_value, false)
        }

        5 => {
            // e.g. let $var = $10
            let validate_num_re = r#"let +(const +) *(\$[A-Za-z]\w*)( *= *\$\d[\d_]*)?"#;
            
            // check if the variable is already declared.
            let var_name = SinglePattern::refr_name(tokens[2]).unwrap();
            if already_declared.contains(&var_name) {
                panic!("Variable {} on line {} is already declared!", var_name, line.num);
            }

            // if numeral literal was present, use it as default value.
            let mut default_value: i16 = 0;
            if validate(validate_num_re, &line.content) {
                default_value = patterns::num_lit_to_i16(tokens[4]).unwrap();
            }
            
            (var_name, default_value, true)
        }

        _ => {
            panic!("Invalid variable declaration on line {}!", line.num);
        }
    }
}

pub fn data_initialization(lines: &[Line]) -> Vec<(String, i16, bool)> {
    let mut declared_names = Vec::new();
    let mut declared_variables = Vec::new();

    for line in lines.iter() {
        if line.content.is_empty() {
            continue;
        }

        if line.content.starts_with("let ") {
            let var = let_handler(line, &declared_names);
            declared_names.push(var.0.clone());
            declared_variables.push(var);
            continue;
        }
        
        for var in fetch_vars(&line) {
            if !declared_names.contains(&var) {
                panic!("Variable {} on line {}, used without declaration!", var, line.num);
            }
        }
    }

    declared_variables
}