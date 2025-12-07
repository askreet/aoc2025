use crate::shared::*;
use std::{
    fs::{File, read_to_string},
    io::{BufRead, BufReader, Lines},
};

pub fn input_lines(day: u8) -> Result<std::io::Lines<BufReader<File>>> {
    let path = format!("inputs/{}.txt", day);

    match std::fs::File::open(&path) {
        Ok(f) => Ok(BufReader::new(f).lines()),
        Err(e) => Err(Error::from(format!("failed to open {path}: {e}"))),
    }
}

pub fn input(day: u8) -> Result<String> {
    let path = format!("inputs/{}.txt", day);

    match read_to_string(&path) {
        Ok(v) => Ok(v.trim_end().to_owned()),
        Err(e) => err(&format!("failed to read file {path}: {e}")),
    }
}
