//! # Advent Of Code 2024
//!
//! Solutions in Rust.
//!
//! This module contains the general utilities that are not associated to
//! the challenge of a particular day, such as reading input data from a
//! file

use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;
pub mod day_14;
pub mod day_17;
pub mod day_18;
pub mod day_22;
//pub mod day_23;
pub mod day_24;
pub mod day_25;

/// Read the data from the input file.
///
/// The path of the file is given by `path`.
///
/// The type into which each line shall be parsed
/// is given by `T`.
pub fn read_data<T, P: AsRef<Path>>(path: P) -> io::Result<Vec<T>>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let f = File::open(path)?;
    let vec = BufReader::new(f)
        .lines()
        .map(|l| l.unwrap().parse::<T>().unwrap())
        .collect();

    Ok(vec)
}
