/*!
 * A dictionary search tool.
 *
 * Copyright (C) 2023-2024 kaoru  <https://www.tetengo.org/>
 */

use std::cmp::min;
use std::env;
use std::fs::File;
use std::io::{stdin, Read};
use std::path::Path;
use std::process::exit;

use anyhow::Result;

use tetengo_trie::{MemoryStorage, Trie, ValueDeserializer};

fn main() {
    if let Err(e) = main_core() {
        eprintln!("Error: {}", e);
        exit(1);
    }
}

fn main_core() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() <= 2 {
        eprintln!("Usage: search_dict UniDic_lex.csv trie.bin");
        return Ok(());
    }

    let lex_csv = load_lex_csv(Path::new(&args[1]))?;
    let trie = load_trie(Path::new(&args[2]))?;

    loop {
        eprint!(">> ");
        let mut line = String::new();
        let read_length = stdin().read_line(&mut line)?;
        if read_length == 0 {
            break;
        }
        if line.is_empty() {
            continue;
        }

        line = line.trim_end().to_string();
        let found = match trie.find(&line)? {
            Some(found) => found,
            None => {
                println!("ERROR: Not found.");
                continue;
            }
        };

        found.iter().for_each(|e| {
            let (offset, length) = *e;
            print!("{}", substring_view(&lex_csv, offset, length));
        });
    }
    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum DictSearchingError {
    #[error("Can't read the whole of lex.csv file.")]
    CantReadWholeOfLexCsvFile,
}

fn load_lex_csv(lex_csv_path: &Path) -> Result<String> {
    let mut file = File::open(lex_csv_path)?;

    let lex_csv_size = file.metadata()?.len();

    let mut buffer = String::new();
    let read_length = file.read_to_string(&mut buffer)?;
    if read_length != lex_csv_size as usize {
        return Err(DictSearchingError::CantReadWholeOfLexCsvFile.into());
    }
    Ok(buffer)
}

type DictTrie = Trie<String, Vec<(usize, usize)>>;

fn load_trie(trie_path: &Path) -> Result<DictTrie> {
    let mut file = File::open(trie_path)?;

    let mut value_deserializer = ValueDeserializer::new(Box::new(deserialize_value));
    let storage = Box::new(MemoryStorage::new_with_reader(
        &mut file,
        &mut value_deserializer,
    )?);
    let trie = DictTrie::builder_with_storage(storage).build();
    Ok(trie)
}

const VALUE_CAPACITY: usize = 4usize;

fn deserialize_value(bytes: &[u8]) -> Result<Vec<(usize, usize)>> {
    let mut byte_offset = 0usize;

    let size = deserialize_usize(bytes, &mut byte_offset)?;
    let mut vps = Vec::with_capacity(size);
    for _ in 0..min(size, VALUE_CAPACITY) {
        vps.push(deserialize_pair_of_usize(bytes, &mut byte_offset)?);
    }
    (VALUE_CAPACITY..size).for_each(|_| {
        vps.push((0, 0));
    });

    Ok(vps)
}

fn deserialize_pair_of_usize(bytes: &[u8], byte_offset: &mut usize) -> Result<(usize, usize)> {
    let first = deserialize_usize(bytes, byte_offset)?;
    let second = deserialize_usize(bytes, byte_offset)?;
    Ok((first, second))
}

fn deserialize_usize(bytes: &[u8], byte_offset: &mut usize) -> Result<usize> {
    let mut value = 0usize;
    (0..size_of::<u32>()).for_each(|i| {
        value <<= 8;
        value |= bytes[*byte_offset + i] as usize;
    });
    *byte_offset += size_of::<u32>();
    Ok(value)
}

fn substring_view(sv: &str, offset: usize, length: usize) -> &str {
    if offset == 0 && length == 0 {
        return "(truncated)\n";
    }
    &sv[offset..offset + length]
}
