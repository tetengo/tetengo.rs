/*!
 * A vocabulary.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

/**
 * A vocabulary.
 */
pub trait Vocabulary {
    /*
        /*!
            \brief Creates a vocabulary.
        */
        vocabulary();
    */
    /*
        /*!
            \brief Finds entries.

            \param key A key.

            \return Entry views.
        */
        [[nodiscard]] std::vector<entry_view> find_entries(const input& key) const;
    */
    /*
        /*!
            \brief Finds a connection between an origin node and a destination entry.

            \param from An origin node.
            \param to   A destination entry.

            \return A connection between the origin node and the destination entry.
        */
        [[nodiscard]] connection find_connection(const node& from, const entry_view& to) const;
    */
}
