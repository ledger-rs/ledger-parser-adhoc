# ledger-parser-adhoc
An ad-hoc rewrite of the Ledger parser

# Intent and Purpose

The idea behind this version of the parser is to create a custom parser from scratch, without the limitations imposed by differences between C++ and Rust.

This is a custom parser, based on the original Ledger parser in C++. Only the minimal functionality is intended to be ported to get a working version.

The intended goal is to have a minimal Ledger-syntax parser that can be compiled into WASM and used in a web application.

It is also supposed to parse some basic set of Ledger functionality and provide a data structure back. It is a library, not an executable. Although, there could be an executable built for the purpose of running tests.

# Progress Plan

The initial step is to get just the minimal structure from Ledger and have a minimal working version, that can parse a very basic transaction (in `tests/first.ledger`).

Expand from there to parse more of the Ledger syntax.

Use existing tests from Ledger and Ledger2Beancount to confirm that the input is parsed.

# Ledger Documentation

The guidelines are available at [Ledger for Developers](https://ledger-cli.org/doc/ledger3.html#Ledger-for-Developers).
There are the explanations for the main objects and functioning of the system. The core concepts should be mirrored in the Rust implementation.

# Conclusion

The rewrite goal is proving elusive as a huge code structure from Ledger still needs to be ported over. Once the parsing starts, the work with references prove to be too much of a mess due to Rust's limits on fields and references.

Try using Box<> more and passing references.

It makes no sense to have just the parser, since the model structure is highly interwoven into the parser. The reporting components would depend on this and it makes sense to include the full Ledger functionality in the same library.
