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

use tetengo_trie::{Serializer, StringSerializer, Trie};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() <= 2 {
        eprintln!("Usage: make_dict UniDic_lex.csv trie.bin");
        return;
    }

    let word_offset_map = match load_lex_csv(Path::new(&args[1])) {
        Ok(word_offset_map) => word_offset_map,
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
    };
    let _trie = match build_trie(word_offset_map) {
        Ok(trie) => trie,
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

/*
class trie_building_observer
{
public:
    trie_building_observer() : m_index{ 0 } {}

    void operator()(const std::string_view& key)
    {
        if (m_index % 10000 == 0)
        {
            std::cerr << boost::format{ "%8d: %s" } % m_index % encode_for_print(key) << "    \r" << std::flush;
        }
        ++m_index;
    }

private:
    std::size_t m_index;
};
*/

type DictTrie = Trie<String, Vec<(usize, usize)>>;

fn build_trie(word_offset_map: WordOffsetMap) -> Result<DictTrie> {
    eprintln!("Building trie...");
    let word_offset_vector = word_offset_map.into_iter().collect::<Vec<_>>();
    let trie = DictTrie::builder()
        .elements(word_offset_vector)
        .key_serializer(StringSerializer::new(true))
        .build();
    eprintln!("Done.        ");
    trie
}

/*
std::unique_ptr<tetengo::trie::trie<std::string_view, std::vector<std::pair<std::size_t, std::size_t>>>>
build_trie(const std::unordered_map<std::string, std::vector<std::pair<std::size_t, std::size_t>>>& word_offset_map)
{
    std::cerr << "Building trie..." << std::endl;
    auto p_trie =
        std::make_unique<tetengo::trie::trie<std::string_view, std::vector<std::pair<std::size_t, std::size_t>>>>(
            std::make_move_iterator(std::begin(word_offset_map)),
            std::make_move_iterator(std::end(word_offset_map)),
            tetengo::trie::default_serializer<std::string_view>{ true },
            tetengo::trie::trie<std::string_view, std::vector<std::pair<std::size_t, std::size_t>>>::
                building_observer_set_type{ trie_building_observer{}, []() {} });
    std::cerr << "Done.        " << std::endl;
    return p_trie;
}
*/

/*
std::vector<char> serialize_size_t(const std::size_t s)
{
    assert(s <= std::numeric_limits<std::uint32_t>::max());

    std::vector<char> serialized(sizeof(std::uint32_t), '\0');

    for (auto i = static_cast<std::size_t>(0); i < sizeof(std::uint32_t); ++i)
    {
        serialized[i] = (s >> ((sizeof(std::uint32_t) - i - 1) * 8)) & 0xFF;
    }

    return serialized;
}
*/
/*
std::vector<char> serialize_pair_of_size_t(const std::pair<std::size_t, std::size_t>& ps)
{
    std::vector<char> serialized{};
    serialized.reserve(sizeof(std::uint32_t) * 2);

    const auto serialized_offset = serialize_size_t(ps.first);
    serialized.insert(std::end(serialized), std::begin(serialized_offset), std::end(serialized_offset));
    const auto serialized_length = serialize_size_t(ps.second);
    serialized.insert(std::end(serialized), std::begin(serialized_length), std::end(serialized_length));

    return serialized;
}
*/
/*
std::vector<char> serialize_vector_of_pair_of_size_t(const std::vector<std::pair<std::size_t, std::size_t>>& vps)
{
    std::vector<char> serialized{};
    serialized.reserve(serialized_value_size);

    const auto serialized_size = serialize_size_t(std::size(vps));
    serialized.insert(std::end(serialized), std::begin(serialized_size), std::end(serialized_size));
    for (auto i = static_cast<std::size_t>(0); i < value_capacity; ++i)
    {
        if (i < std::size(vps))
        {
            const auto& ps = vps[i];
            const auto  serialized_element = serialize_pair_of_size_t(ps);
            serialized.insert(std::end(serialized), std::begin(serialized_element), std::end(serialized_element));
        }
        else
        {
            const auto serialized_element =
                serialize_pair_of_size_t(std::make_pair<std::size_t, std::size_t>(0, 0));
            serialized.insert(std::end(serialized), std::begin(serialized_element), std::end(serialized_element));
        }
    }

    return serialized;
}
*/
/*
std::vector<char> serialize_value(const std::any& value)
{
    const auto* const p_value = std::any_cast<std::vector<std::pair<std::size_t, std::size_t>>>(&value);
    assert(p_value);
    return serialize_vector_of_pair_of_size_t(*p_value);
}
*/
/*
void serialize_trie(
    const tetengo::trie::trie<std::string_view, std::vector<std::pair<std::size_t, std::size_t>>>& trie_,
    const std::filesystem::path&                                                                   trie_bin_path)
{
    std::cerr << "Serializing trie..." << std::endl;
    std::ofstream output_stream{ trie_bin_path, std::ios_base::binary };
    if (!output_stream)
    {
        throw std::ios_base::failure{ "Can't open the output file." };
    }
    const tetengo::trie::value_serializer serializer{ serialize_value, serialized_value_size };
    trie_.get_storage().serialize(output_stream, serializer);
    std::cerr << "Done.        " << std::endl;
}
*/
