/*!
 * A dictionary search tool.
 *
 * Copyright (C) 2023-2025 kaoru  <https://www.tetengo.org/>
 */

use std::cmp::min;
use std::env;
use std::fs::File;
use std::io::{Read, stdin};
use std::path::Path;
use std::process::exit;

use tetengo_trie::{Error, MemoryStorage, Trie, ValueDeserializer};

fn main() {
    if let Err(e) = main_core() {
        eprintln!("Error: {}", e);
        exit(1);
    }
}

fn main_core() -> Result<(), Error> {
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
        let read_length = stdin()
            .read_line(&mut line)
            .map_err(|e| Error::InternalError(e.into()))?;
        if read_length == 0 {
            break;
        }
        if line.is_empty() {
            continue;
        }

        line = line.trim_end().to_string();
        let Some(found) = trie.find(&line)? else {
            println!("ERROR: Not found.");
            continue;
        };

        found.iter().for_each(|e| {
            let (offset, length) = *e;
            print!("{}", substring_view(&lex_csv, offset, length));
        });
    }
    Ok(())
}

fn load_lex_csv(lex_csv_path: &Path) -> Result<String, Error> {
    let mut file = File::open(lex_csv_path).map_err(|e| Error::InternalError(e.into()))?;

    let lex_csv_size = file
        .metadata()
        .map_err(|e| Error::InternalError(e.into()))?
        .len();

    let mut buffer = String::new();
    let read_length = file
        .read_to_string(&mut buffer)
        .map_err(|e| Error::InternalError(e.into()))?;
    if read_length != usize::try_from(lex_csv_size).map_err(|e| Error::InternalError(e.into()))? {
        return Err(Error::UnexpectedEof);
    }
    Ok(buffer)
}

type DictTrie = Trie<String, Vec<(usize, usize)>>;

fn load_trie(trie_path: &Path) -> Result<DictTrie, Error> {
    let mut file = File::open(trie_path).map_err(|e| Error::InternalError(e.into()))?;

    let mut value_deserializer = ValueDeserializer::new(Box::new(deserialize_value));
    let storage = Box::new(MemoryStorage::new_with_reader(
        &mut file,
        &mut value_deserializer,
    )?);
    let trie = DictTrie::builder_with_storage(storage).build();
    Ok(trie)
}

const VALUE_CAPACITY: usize = 4usize;

fn deserialize_value(bytes: &[u8]) -> Result<Vec<(usize, usize)>, Error> {
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

fn deserialize_pair_of_usize(
    bytes: &[u8],
    byte_offset: &mut usize,
) -> Result<(usize, usize), Error> {
    let first = deserialize_usize(bytes, byte_offset)?;
    let second = deserialize_usize(bytes, byte_offset)?;
    Ok((first, second))
}

fn deserialize_usize(bytes: &[u8], byte_offset: &mut usize) -> Result<usize, Error> {
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
