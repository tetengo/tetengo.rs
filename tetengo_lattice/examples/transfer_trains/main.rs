/*!
 * A train transfer guide.
 *
 * Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>
 */

mod timetable;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::exit;

use anyhow::Result;
use timetable::Timetable;

fn main() {
    if let Err(e) = main_core() {
        eprintln!("Error: {}", e);
        exit(1);
    }
}

fn main_core() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() <= 1 {
        eprintln!("Usage: transfer_trains timetable.txt");
        return Ok(());
    }

    let _timetable = Timetable::new(create_reader(Path::new(&args[1]))?);
    Ok(())
}

fn create_reader(path: &Path) -> Result<Box<dyn BufRead>> {
    let reader = BufReader::new(File::open(path)?);
    Ok(Box::new(reader))
}
