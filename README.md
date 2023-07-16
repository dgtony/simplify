# Simplify
CLI tool for simplifying complex boolean expressions with many (well, not so many) variables.
Under the hood it uses Quine-McCluskey algorithm for finding minimal form of a boolean expression.

## Language
Target expression expected as a string argument consisting of variables, operators and parentheses (both square and round).
Expression may also contain whitespaces, tabs and newlines, which will be ignored by the parser.

Recognized boolean operators:
* `&` / `*` - **AND**
* `|` / `+` - **OR**
* `!` - **NOT** (prefix)

Variable names may contain latin letters, numbers and underscores.

## Installation
* get Rust toolchain
* `cargo install`

## Usage
Simple expressions often become even simpler.
```cmd
$ simplify '([A | B] & (!B & C) | C)'
[OK] 1 solution(s) found
=> C
```

Some expressions may not depend on its arguments at all.
```cmd
$ simplify 'v_1 + !vs_qsd_912 + some_other_var + !(vs_qsd_912 * v_1)'
[OK] 1 solution(s) found
=> true
```

While some other complex expressions may produce more than a single minimal form.
```cmd
$ simplify '!a & !b & !c & d | !a & b & !c & d | a & b & c & d
| !a & !b & !c & !d | !a & !b & c & d | !a & b & c & !d | !a & b & c & !d
| !a & b & c & d | a & b & !c & !d | a & !b & c & !d | a & !b & c & d
| a & !b & !c & !d | a & b & c & !d | !a & !b & c & !d'
[OK] 2 solution(s) found
=> !b & !d | a & !d | !a & d | c
=> a & !d | !a & d | !a & !b | c
```
