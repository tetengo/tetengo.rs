mod usage {
    #[test]
    fn viterbi() {}

    /*
        void viterbi()
        {
            /*
                Makes the following lattice and searches it.

                        /-----[ab:AwaBizan]-----\
                       /  (7)      (9)      (1)  \
                      /                           \
                     /       (2)   (4)   (7)       \
                [BOS]-----[a:Alpha]---[b:Bravo]-----[EOS]
                     \ (3)         \ /(1)      (2) /
                      \(1)          X             /(6)
                       \           / \(5)        /
                        `-[a:Alice]---[b:Bob]---'
                             (1)   (9)  (8)
                Path                         Cost
                [BOS]-[Alice]-[Bravo]-[EOS]  1+1+1+7+2=12
                [BOS]---[AwaBizan]----[EOS]  7+9+1    =17
                [BOS]-[Alpha]-[Bravo]-[EOS]  3+2+4+7+2=18
                [BOS]-[Alpha]-[Bob]---[EOS]  3+2+5+8+6=24
                [BOS]-[Alice]-[Bob]---[EOS]  1+1+9+8+6=25
            */

            // Builds a vocabulary.
            const auto p_vocabulary = build_vocabulary();

            // Creates an object for a lattice.
            tetengo::lattice::lattice lattice_{ *p_vocabulary };

            // Enters key characters to construct the lattice.
            lattice_.push_back(std::make_unique<tetengo::lattice::string_input>("a"));
            lattice_.push_back(std::make_unique<tetengo::lattice::string_input>("b"));

            // Finishes the lattice construction.
            const auto eos_and_preceding_costs = lattice_.settle();

            // Creates an iterator to enumerate the paths in the lattice.
            tetengo::lattice::n_best_iterator first{ lattice_,
                                                     eos_and_preceding_costs.first,
                                                     std::make_unique<tetengo::lattice::constraint>() };

            // Enumerates the paths.
            std::vector<std::string> paths{};
            std::for_each(
                std::move(first), tetengo::lattice::n_best_iterator{}, [&paths](const tetengo::lattice::path& path_) {
                    paths.push_back(to_string(path_));
                });

            static const std::vector<std::string> expected{
                // clang-format off
                "[BOS]-[Alice]-[Bravo]-[EOS] (12)",
                "[BOS]-[AwaBizan]-[EOS] (17)",
                "[BOS]-[Alpha]-[Bravo]-[EOS] (18)",
                "[BOS]-[Alpha]-[Bob]-[EOS] (24)",
                "[BOS]-[Alice]-[Bob]-[EOS] (25)",
                // clang-format on
            };
            assert(paths == expected);
        }
    */
    /*
        std::string value_of(const tetengo::lattice::entry_view& entry)
        {
            // The value is stored in the std::any object.
            return entry.value()->has_value() ? std::any_cast<std::string>(*entry.value()) : std::string{};
        }
    */
    /*
        std::unique_ptr<tetengo::lattice::vocabulary> build_vocabulary()
        {
            // The contents of the vocabulary.
            static const std::vector<tetengo::lattice::entry> entries{
                // clang-format off
                { std::make_unique<tetengo::lattice::string_input>("a"), std::string{ "Alpha" }, 2 },
                { std::make_unique<tetengo::lattice::string_input>("b"), std::string{ "Bravo" }, 7 },
                { std::make_unique<tetengo::lattice::string_input>("a"), std::string{ "Alice" }, 1 },
                { std::make_unique<tetengo::lattice::string_input>("b"), std::string{ "Bob" }, 8 },
                { std::make_unique<tetengo::lattice::string_input>("ab"), std::string{ "AwaBizan" }, 9 },
                // clang-format on
            };
            std::vector<std::pair<std::string, std::vector<tetengo::lattice::entry>>> entry_mappings{
                // clang-format off
                { "a", { entries[0], entries[2] } },
                { "b", { entries[1], entries[3] } },
                { "ab", { entries[4] }},
                // clang-format on
            };
            std::vector<std::pair<std::pair<tetengo::lattice::entry, tetengo::lattice::entry>, int>> connections{
                // clang-format off
                { { tetengo::lattice::entry::bos_eos(), entries[0] }, 3 },
                { { tetengo::lattice::entry::bos_eos(), entries[2] }, 1 },
                { { entries[0], entries[1] }, 4 },
                { { entries[2], entries[1] }, 1 },
                { { entries[0], entries[3] }, 5 },
                { { entries[2], entries[3] }, 9 },
                { { entries[1], tetengo::lattice::entry::bos_eos() }, 2 },
                { { entries[3], tetengo::lattice::entry::bos_eos() }, 6 },
                { { tetengo::lattice::entry::bos_eos(), entries[4] }, 7 },
                { { entries[4], tetengo::lattice::entry::bos_eos() }, 1 },
                // clang-format on
            };
    */
    /*
            // Returns a vocabulary implemented with hash tables.
            return std::make_unique<tetengo::lattice::unordered_map_vocabulary>(
                std::move(entry_mappings),
                std::move(connections),
                [](const tetengo::lattice::entry_view& entry) {
                    return (entry.p_key() ? entry.p_key()->hash_value() : 0) ^ std::hash<std::string>{}(value_of(entry));
                },
                [](const tetengo::lattice::entry_view& entry1, const tetengo::lattice::entry_view& entry2) {
                    return ((!entry1.p_key() && !entry2.p_key()) ||
                            (entry1.p_key() && entry2.p_key() && *entry1.p_key() == *entry2.p_key())) &&
                           (value_of(entry1) == value_of(entry2));
                });
        }
    */
    /*
        std::string value_of(const tetengo::lattice::node& node_, const bool first)
        {
            if (node_.value().has_value())
            {
                // The value is stored in the std::any object.
                return std::any_cast<std::string>(node_.value());
            }
            else if (first)
            {
                return "BOS";
            }
            else
            {
                return "EOS";
            }
        }
    */
    /*
       std::string to_string(const tetengo::lattice::path& path_)
       {
           // Each path object holds the nodes that make up itself, and the whole cost.
           std::string result{};
           for (const auto& node: path_.nodes())
           {
               if (!std::empty(result))
               {
                   result += "-";
               }
               result += "[" + value_of(node, std::empty(result)) + "]";
           }
           result += " (" + std::to_string(path_.cost()) + ")";
           return result;
       }
    */
}
