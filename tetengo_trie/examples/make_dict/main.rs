/*!
 * A dictionary building tool.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use std::env;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() <= 2 {
        eprintln!("Usage: make_dict UniDic_lex.csv trie.bin");
        // return;
    }

    // const auto word_offset_map = load_lex_csv(argv[1]);
    // const auto p_trie = build_trie(word_offset_map);
    // serialize_trie(*p_trie, argv[2]);
}
