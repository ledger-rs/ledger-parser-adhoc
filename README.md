# ledger-parser-adhoc
An ad-hoc rewrite of the Ledger parser

This parser is a custom parser, based on the original Ledger parser in C++. Only the minimal functionality is intended to be ported to get a working version.

The intended goal is to have a minimal Ledger-syntax parser that can be compiled into WASM and used in a web application.

It is also supposed to parse some basic set of Ledger functionality and provide a data structure back. It is a library, not an executable. Although, there could be an executable built for the purpose of running tests.

# Ledger Documentation

Some guidelines available at [Ledger for Developers](https://ledger-cli.org/doc/ledger3.html#Ledger-for-Developers).

# Purpose

The idea behind this version of the parser is to get just the minimal structure from Ledger and have a minimal working version, that can parse a very basic transaction (in `tests/first.ledger`).

# Conclusion

This goal is proving elusive as a huge structure from Ledger still needs to be ported over. Once the parsing starts, the references prove to be too much of a mess due to Rust's limits on fields and references.

Try using Box<>.

