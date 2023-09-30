/*!
 * A dictionary building tool.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::exit;

use anyhow::Result;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() <= 2 {
        eprintln!("Usage: make_dict UniDic_lex.csv trie.bin");
        return;
    }

    let _word_offset_map = match load_lex_csv(Path::new(&args[1])) {
        Ok(word_offset_map) => word_offset_map,
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
    };

    // const auto word_offset_map = load_lex_csv(argv[1]);
    // const auto p_trie = build_trie(word_offset_map);
    // serialize_trie(*p_trie, argv[2]);
}

#[derive(thiserror::Error, Debug)]
enum DictMakingError {
    #[error("Invalid UniDic lex.csv format.")]
    InvalidUnidicLexCsvFormat,
}

type WordOffsetMap = HashMap<String, Vec<(usize, usize)>>;

fn load_lex_csv(lex_csv_path: &Path) -> Result<WordOffsetMap> {
    let file = File::open(lex_csv_path)?;

    let mut word_offset_map = WordOffsetMap::new();

    eprintln!("Loading UniDic lex.csv...");
    let mut line_head = 0usize;
    let buf_reader = BufReader::new(file);
    for (i, line) in buf_reader.lines().enumerate() {
        let Ok(line) = line else {
            eprintln!("{:8}: Can't read this line.", i);
            return Err(DictMakingError::InvalidUnidicLexCsvFormat.into());
        };
        if line.is_empty() {
            line_head += line.len() + 1;
            continue;
        }
        let elements = split(&line, ',');
        if elements.len() != 33 {
            eprintln!("{:8}: {}", i, elements[0]);
            return Err(DictMakingError::InvalidUnidicLexCsvFormat.into());
        }

        if elements[16] == "記号" && elements[23] == "補助記号" {
            insert_word_offset_to_map(elements[0], line_head, line.len() + 1, &mut word_offset_map);
        } else {
            insert_word_offset_to_map(
                elements[12],
                line_head,
                line.len() + 1,
                &mut word_offset_map,
            );
            insert_word_offset_to_map(
                elements[24],
                line_head,
                line.len() + 1,
                &mut word_offset_map,
            );
        }

        if i % 10000 == 0 {
            eprint!("{:8}: {}    \r", i, elements[0]);
        }

        line_head += line.len() + 1;
    }
    eprintln!("Done.        ");

    Ok(word_offset_map)
}

fn split(string: &str, delimiter: char) -> Vec<&str> {
    let mut elements = Vec::new();

    let mut first = 0usize;
    loop {
        if first < string.len() && string[first..].starts_with('"') {
            if let Some(length) = string[first + 1..].find('"') {
                let last = first + 1 + length;
                elements.push(&string[first + 1..last]);
                debug_assert!(string[last + 1..].starts_with(delimiter));
                first = last + 2;
            } else {
                elements.push(&string[first + 1..]);
                break;
            }
        } else if let Some(length) = string[first..].find(delimiter) {
            let last = first + length;
            elements.push(&string[first..last]);
            first = last + 1;
        } else {
            elements.push(&string[first..]);
            break;
        }
    }

    elements
}

fn insert_word_offset_to_map(key: &str, offset: usize, length: usize, map: &mut WordOffsetMap) {
    let i_value = map.entry(key.to_string()).or_insert_with(Vec::new);
    if i_value.iter().any(|&(o, l)| o == offset && l == length) {
        return;
    }
    if i_value.len() < 100 {
        i_value.push((offset, length));
    } else {
        i_value.push((0, 0));
    }
}
