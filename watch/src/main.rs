use clap::Parser;

#[derive(Parser)]
///入力された秒を`時:分:秒`の形式に変換する。
struct Second {
    ///秒
    seconds: u32,
}

impl Second {
    fn format(&self) -> String {
        match self.seconds {
            0..=86400 => {
                let tap = self.calc();
                format!("{}:{}:{}", tap.0, tap.1, tap.2)
            }
            _ => "enter a number between 0 and 86400.".to_string(),
        }
    }

    fn calc(&self) -> (u32, u32, u32) {
        let mut num = self.seconds;
        let mut tap = (0, 0, 0);

        if num >= 3600 {
            tap.0 = num / 3600;
            num %= 3600;
        }
        if num >= 60 {
            tap.1 = num / 60;
            num %= 60;
        }
        tap.2 = num;
        tap
    }
}

fn main() {
    let number = Second::parse();
    println!("{}", number.format());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let num = Second { seconds: 46979 };
        assert_eq!(num.format(), "13:2:59");
    }
}