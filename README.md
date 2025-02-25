# Check Digits

A Rust library and binary to determine if a string of digits can be considered equal after certain operations, based on binomial coefficient properties modulo 2 and 5.

## Overview

This project implements a solution to the "Check if Digits are Equal in String After Operations II" problem. It computes the sum of differences between consecutive digits, weighted by binomial coefficients \(\binom{n-2}{i}\), and checks if the result is zero modulo both 2 and 5. The implementation is optimized for runtime performance with:

- No heap allocations beyond the input string.
- Precomputed modular inverses for modulo 5.
- Efficient incremental updates of binomial coefficients.
- Bitwise operations for modulo 2 where applicable.

## Installation

Clone the repository and build with Cargo:

```bash
git clone https://github.com/aliezzahn/check-digits.git
cd check-digits
cargo build --release
```

### Prerequisites

- Rust (stable, edition 2021) installed via [rustup](https://rustup.rs/).
- Git for cloning the repository.

## Usage

### As a Binary

Run the prebuilt example:

```bash
cargo run --release
```

Expected output:

```
Input: 3902, Result: true
Input: 34789, Result: false
Input: 323, Result: true
Input: 355, Result: false
Input: 242, Result: true
Input: 797, Result: true
Input: 9206, Result: false
```

### As a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
check-digits = { git = "https://github.com/aliezzahn/check-digits.git" }
```

Use in your code:

```rust
use check_digits::Solution;

fn main() {
    let result = Solution::has_same_digits("323".to_string());
    println!("Can digits be equal? {}", result); // true
}
```

## API Documentation

The library provides a single public method:

- `Solution::has_same_digits(s: String) -> bool`: Checks if the digit sequence satisfies the binomial condition.

Full documentation is available in the source code using Rustdoc. Generate and view it locally:

```bash
cargo doc --open
```

## Test Cases

The library has been verified against the following inputs:

- `"3902"` → `true`
- `"34789"` → `false`
- `"323"` → `true`
- `"355"` → `false`
- `"242"` → `true`
- `"797"` → `true`
- `"9206"` → `false`

Run tests to verify:

```bash
cargo test
```

## Performance

- **Time Complexity**: \( O(n) \) per modulus check, where \( n \) is the string length, with optimized constant factors.
- **Space Complexity**: \( O(1) \) heap usage, leveraging stack variables and string slices.

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature/your-feature`).
3. Commit your changes (`git commit -am 'Add your feature'`).
4. Push to the branch (`git push origin feature/your-feature`).
5. Open a Pull Request.

### Development

- Format code: `cargo fmt`
- Check linting: `cargo clippy`
- Run tests: `cargo test`

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by combinatorial problems involving binomial coefficients.
- Built with Rust for performance and safety.
