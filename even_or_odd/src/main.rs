use clap::Parser;

#[derive(Parser)]
/// 偶数(Even)か奇数(Odd)かを判定する。
struct Number {
    /// 判定をする自然数
    number: u32,
}

impl Number {
    fn check(&self) -> &str {
        match self.number % 2 {
            0 => "Even",
            _ => "Odd",
        }
    }
}

fn main() {
    let number = Number::parse();
    println!("{}", number.check());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let num = Number { number: 2 };
        assert_eq!(num.check(), "Even");
        let num = Number { number: 3 };
        assert_eq!(num.check(), "Odd");
    }
}
