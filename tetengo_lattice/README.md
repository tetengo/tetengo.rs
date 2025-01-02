tetengo Lattice 1.3.1
=====================

A Viterbi search library.

The Viterbi search is a dynamic programming algorithm.
It finds the most likely path in a lattice consisting of observed event nodes.

This library also provides the A* search algorithm for the lattice created by
the Viterbi search.

- [Detailed description](https://docs.rs/tetengo_lattice/1.3.1/tetengo_lattice/)

How to Use
----------

Execute the `cargo add` command to add the "tetengo_lattice" library to your
cargo package.

An entry for "tetengo_lattice" will be added to the "dependencies" section of
Cargo.toml.

- On Windows:
  - ```bat
    X:>cd \path\to\your\package
    X:>cargo add tetengo_lattice
    ```
- On Linux:
  - ```shell-session
    $ cd /path/to/your/package
    $ cargo add tetengo_lattice
    ```

See
[the cargo document](https://doc.rust-lang.org/cargo/commands/cargo-add.html)
for details.

Source Files
------------

The source files for this library are available on GitHub.

- [https://github.com/tetengo/tetengo.rs](https://github.com/tetengo/tetengo.rs)


---

Copyright (C) 2023-2025 kaoru  <https://www.tetengo.org/>

This product is released under the MIT license.
See [the LICENSE
file](https://github.com/tetengo/tetengo.rs/blob/main/LICENSE) for details.
