# Rusty Utils

[![Crate](https://img.shields.io/crates/v/rusty_utils.svg)](https://crates.io/crates/rusty_utils)
![Test](https://img.shields.io/badge/tests-passing-green)
![Docs](https://img.shields.io/badge/docs-passing-green)

A Rust package that provides utility functions inspired by or ported from other programming languages
to make transitioning from JS or Python a lot easier.
Some of those functions are:

- `ternary_operator()`: a function to emulate the ternary operator that is not present in Rust.
- `reverse_string()`: inspired by the `array.reverse()` method available in JS.
- `concat_arrays()`: a fast way to add all the values present in multiples arrays into a single vector.

There's more detailed documentation available [here](https://docs.rs/rusty_utils/latest/rusty_utils/).<br>
More functions and methods from other languages will be added in the future.

## Usage

Add this to your `cargo.toml`:

```rust
[dependencies]
rusty_utils = "*"    
```

You can choose replace the asterisk with your prefered version number, or keep it to use the latest version.

## Contributing

If you see a bug, have some idea that would be cool to add, or want to improve the looks of the documentation:

- Fork the repository.
- Add you contribution in a new branch.
- Send a pull request.

All contributions are welcome, don't be shy!

## License

This project is licensed under the MIT License.
