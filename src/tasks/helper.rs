use std::fs::File;
use std::{fs, io};
use std::io::{BufRead, BufReader, Lines};
use std::process::exit;

/*
 * Reads a file and returns a vector of strings
 * Each string is a line in the file
 */
pub fn get_lines(file: &str) -> Vec<String>
{
    let file = File::open(file).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
