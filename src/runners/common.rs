use std::fs::File;
use std::io::{Read, Result};

pub fn get_input(basename: &str) -> Result<String> {
    let filepath = format!("./inputs/{}.txt", basename);
    let mut f = File::open(filepath)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    Ok(contents)
}