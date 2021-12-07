# Advent of Code work
This is my implementations of [Advent of Code](https://adventofcode.com/) in
Rust.

## License
Copyright (C) 2017-2021 James Cowgill

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.

## Test Data Layout
Directory layout:
```
tests/data/
  <YEAR>/
    <DAY>/
      <TEST_NAME>.in[.<STAR>]
      <TEST_NAMR>.out.<STAR>
```

Where:
- `YEAR` is a zero padded integer with width 4.
- `DAY` is a zero padded integer with width 2 (eg `02`).
- `TEST_NAME` is any string not containing period characters.
- Files with an extension containing `in` are input test files. The
  `STAR` is an optional integer star number the test applies to.
- Files with an extension containing `out` are output test files. The
  star number is required for these files.

### Test file matching
Each output file corresponds to a single test against a specific
star. If it exists, the input file qualified with the same star number
is used, otherwise the unqualified input file is used.

Example file list:
- `a.in`
- `a.in.3`
- `a.out.1`
- `a.out.2`
- `a.out.3`
- `b.in.1`
- `b.out.1`

This creates this list of tests:
- `a.in   -> a.out.1` (star 1)
- `b.in.1 -> b.out.1` (star 1)
- `a.in   -> a.out.2` (star 2)
- `a.in.3 -> a.out.3` (star 3)
