# Simple Calulator
入力した2つの正数と演算子の計算結果を出力します。

演算子
- +(和)
- -(差)
- x(積)
- /(商)

# Requirement
* Rust

# Usage
```bash
cargo run -p simple-calculator -- 1.1 + 1
// 2.1
cargo run -p simple-calculator -- 1.1 - 1
// 0.1
cargo run -p simple-calculator -- 1.1 x 2
// 2.2
cargo run -p simple-calculator -- 1.1 / 2
// 0.550
```