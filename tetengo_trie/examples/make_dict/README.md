make_dict
=========

Makes an index file of UniDic lex.csv.

Synopsis
--------

```sh
make_dict lex.csv dict.bin
```

Description
-----------

Specify existing `lex.csv` as an input file, and `dict.bin` as an output file.

For each line of `lex.csv`, it extracts columns "orth" (12th) and "kana"
(24th), and use them as keys of the line. If the line stands for a symbol,
it extracts a column "surface" (0th) only for a key.

The index is consisted of pairs of key and the positions of the corresponding
lines in `lex.csv`.

The program finally stores the index into `dict.bin`.

### About UniDic

UniDic is an electronic dictionary for Japanese natural language processings.
It is developed by National Institute for Japanese Language and Linguistics.

The dictionary data can be obtained in the following website:

https://unidic.ninjal.ac.jp/download#unidic_bccwj

Download the latest archive file unidic-cwj-x.y.z.zip from the site.

`lex.csv` that make_dict uses is found in the archive.

Return Value
------------

Returns 0 when the program exits successfully.

Returns a non-zero value when some error is happened.

---

Copyright (C) 2023-2025 kaoru  https://www.tetengo.org/
