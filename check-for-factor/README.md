# Check for Factor
整数bが整数aの約数であるかを判定します。

# Requirement
* Rust

# Usage
5が10の約数であるかを判定する。
```bash
cargo run -p check-for-factor -- 10 5
// true
```
3が10の約数であるかを判定する。
```bash
cargo run -p check-for-factor -- 10 3
// false
```