# minigrep

A Rust tutorial (from the Rust book with some slight adjustments) in making minigrep, a simplified version of `grep` (**g**lobally search a **r**egular **e**xpression and **p**rint). 

To run minigrep while developing, can run `cargo run -- <search_string> <file>`. The `--` indicates that the following arguments are for the program rather than for `cargo`. Use the `IGNORE_CASE` environment variable to toggle case insensitivity.
