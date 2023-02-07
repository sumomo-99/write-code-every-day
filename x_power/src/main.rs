use clap::Parser;

#[derive(Parser)]
///自然数AのX乗を計算します。
struct Args {
    ///底
    a: usize,
    ///指数
    x: u32,
}

fn power(a: usize, x: u32) -> Result<usize, &'static str> {
    match a.checked_pow(x) {
        Some(result) => Ok(result),
        None => Err("Overflow"),
    }
}

fn main() {
    let args = Args::parse();
    match power(args.a, args.x) {
        Ok(result) => println!("{}の{}乗は{}です。", args.a, args.x, result),
        Err(msg) => println!("計算できません。{}", msg),
    }
}

#[test]
fn test_power() {
    assert_eq!(9, power(3, 2).unwrap());
    assert_eq!(27, power(3, 3).unwrap());
    assert!(power(10, 100).is_err());
}
