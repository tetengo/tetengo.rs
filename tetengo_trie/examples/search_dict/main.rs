/*!
 * A dictionary search tool.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use std::env;
use std::io::stdin;
use std::process::exit;

use anyhow::Result;

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

    loop {
        eprint!(">> ");
        let mut line = String::new();
        let read_length = stdin().read_line(&mut line)?;
        if read_length == 0 {
            break;
        }
    }
    Ok(())
}

/*
int main(const int argc, char** const argv)
{
    try
    {
        const auto lex_csv = load_lex_csv(argv[1]);
        const auto p_trie = load_trie(argv[2]);

        while (std::cin)
        {
            std::cerr << ">> " << std::flush;
            std::string key{};
            std::getline(std::cin, key);
            if (std::empty(key))
            {
                continue;
            }

            const auto* const p_found = p_trie->find(decode_from_input(key));
            if (!p_found)
            {
                std::cout << encode_for_print("ERROR: Not found.") << std::endl;
                continue;
            }

            for (const auto& e: *p_found)
            {
                std::cout << encode_for_print(substring_view(lex_csv, e.first, e.second));
            }
            std::cout << std::flush;
        }

        return 0;
    }
    catch (const std::exception& e)
    {
        std::cerr << "Error: " << e.what() << std::endl;
        return 1;
    }
    catch (...)
    {
        std::cerr << "Error: unknown error." << std::endl;
        return 1;
    }
}
*/
