transfer_trains
===============

Shows trains suitable for your specified section and time.

Synopsis
--------

```sh
transfer_trains timetable.txt
```

Description
-----------

At first, this program asks you a departure station and time, and an arrival
station.

Then, this program lists trains. Some list items have several trains. That
means you need transferring trains.

The list is ordered by costs. The costs is calculated with the time between
departure and arrival and the transfer count.

### About a timetable data file

Timetable files for this program are 2-d table files.

The first and second lines are a station name list and a station telegram code
list. The first two columns must be blanks in these two lines.

The third and later lines stand for trains. The first row is a train number,
and the second row is a train name. The third and later rows are arrival and
departure times.

Here is the format of arrival and departure time cells:

- `HH:MM/HH:MM`
  - Arrival time/Departure time.
- `HH:MM`
  - Departure time. Arrival time is guessed automatically.
- `HH:MM/`
  - Arrival time only. It means the final destination of the train.
- `-`
  - Passing. The train does not stop at this station.
- Blank
  - Out of the operational section of the train.

There are sample timetable files supplied in the directory where this README.md
exists.

- kagoshima_down.txt
  - The part of Kagoshima line of Kyushu Railway Company in Japan.
- kotoku_up.txt
  - The part of Kotoku line of Shikoku Railway Company in Japan.

Return Value
------------

Returns 0 when the program exits successfully.

Returns a non-zero value when some error is happened.

---

Copyright (C) 2023-2025 kaoru  https://www.tetengo.org/
