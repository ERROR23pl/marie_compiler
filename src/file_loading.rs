use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

use crate::schema::Line;

// opens up a file with name in the argument, and returns list of lines ommiting empty lines
pub fn load_file(file_name: &str) -> Result<Vec<Line>, io::Error> {
    let path = Path::new(file_name);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut lines = Vec::new();
    
    for (num, line) in reader.lines().enumerate() {
        lines.push(Line {
            num: (num as i32) + 1,
            content: (line?).trim().to_string(),
        });
    }
    Ok(lines)
}