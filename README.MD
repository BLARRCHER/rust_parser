# rust_parser

A library (crate) for parsing/serializing/deserializing financial data into several formats, plus separate CLI executables (comparer, converter) that use this library.
Supported formats:
1. csv - a table of bank operations
2. txt - a text format describing a list of operations
3. bin - a binary representation of a list of operations

# Usage examples
1. Tests - `cargo test`
2. Run comparer - `cargo run --bin comparer -- --file1 records_example.bin --format1 bin --file2 records_example.txt --format2 txt`
3. Run converter - `cargo run --bin converter -- --input records_example.bin --input-format bin --output-format txt`
