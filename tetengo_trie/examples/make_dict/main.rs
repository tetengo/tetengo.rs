/*!
 * A dictionary building tool.
 *
 * Copyright (C) 2023-2025 kaoru  <https://www.tetengo.org/>
 */

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use std::path::Path;
use std::process::exit;

use anyhow::Result;

use tetengo_trie::{
    BuldingObserverSet, Error, Serializer, StringSerializer, Trie, ValueSerializer,
};

fn main() {
    if let Err(e) = main_core() {
        eprintln!("Error: {}", e);
        exit(1);
    }
}

fn main_core() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() <= 2 {
        eprintln!("Usage: make_dict UniDic_lex.csv trie.bin");
        return Ok(());
    }

    let word_offset_map = load_lex_csv(Path::new(&args[1]))?;
    let trie = build_trie(word_offset_map)?;
    serialize_trie(&trie, Path::new(&args[2]))?;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
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

        if elements[16] == "記号" && elements[23] == "補助" {
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

const VALUE_CAPACITY: usize = 4usize;

fn insert_word_offset_to_map(key: &str, offset: usize, length: usize, map: &mut WordOffsetMap) {
    let value = map.entry(key.to_string()).or_default();
    if value.iter().any(|&(o, l)| o == offset && l == length) {
        return;
    }
    if value.len() < VALUE_CAPACITY {
        value.push((offset, length));
    } else {
        value.push((0, 0));
    }
}

type DictTrie = Trie<String, Vec<(usize, usize)>>;

fn build_trie(word_offset_map: WordOffsetMap) -> Result<DictTrie, Error> {
    eprintln!("Building trie...");
    let mut word_offset_vector = word_offset_map.into_iter().collect::<Vec<_>>();
    word_offset_vector.sort();
    let mut index = 0usize;
    let trie = DictTrie::builder()
        .elements(word_offset_vector)
        .key_serializer(StringSerializer::new(true))
        .build_with_observer_set(&mut BuldingObserverSet::new(
            &mut |key| {
                if index % 10000 == 0 {
                    eprint!("{:8}: {}    \r", index, String::from_utf8_lossy(key));
                }
                index += 1;
            },
            &mut || {},
        ));
    eprintln!("Done.        ");
    trie
}

const SERIALIZED_VALUE_SIZE: usize = size_of::<u32>() * (1 + 4 * 2);

fn serialize_trie(trie: &DictTrie, trie_bin_path: &Path) -> Result<()> {
    eprintln!("Serializing trie...");
    let file = File::create(trie_bin_path)?;
    let mut buf_writer = BufWriter::new(file);
    let mut serializer = ValueSerializer::new(Box::new(serialize_value), SERIALIZED_VALUE_SIZE);
    trie.storage().serialize(&mut buf_writer, &mut serializer)?;
    eprintln!("Done.        ");
    Ok(())
}

#[allow(clippy::ptr_arg)]
fn serialize_value(vpus: &Vec<(usize, usize)>) -> Vec<u8> {
    let mut serialized = Vec::with_capacity(SERIALIZED_VALUE_SIZE);

    let serialized_size = serialize_usize(vpus.len());
    serialized.extend(serialized_size);

    (0..VALUE_CAPACITY).for_each(|i| {
        if i < vpus.len() {
            let serialized_element = serialize_pair_of_usize(&vpus[i]);
            serialized.extend(serialized_element);
        } else {
            let serialized_element = serialize_pair_of_usize(&(0, 0));
            serialized.extend(serialized_element);
        }
    });

    serialized
}

fn serialize_pair_of_usize(pus: &(usize, usize)) -> Vec<u8> {
    let mut serialized = Vec::with_capacity(size_of::<usize>() * 2);

    let (offset, length) = pus;
    serialized.extend(serialize_usize(*offset));
    serialized.extend(serialize_usize(*length));

    serialized
}

fn serialize_usize(us: usize) -> Vec<u8> {
    debug_assert!(us <= u32::MAX as usize);

    let mut serialized = Vec::from([0u8; size_of::<u32>()]);
    (0..size_of::<u32>()).for_each(|i| {
        serialized[i] = ((us >> ((size_of::<u32>() - i - 1) * 8)) & 0xFF) as u8;
    });
    serialized
}
