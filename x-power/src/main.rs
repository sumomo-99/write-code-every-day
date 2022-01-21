use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "x-power")]
///Calculates Xth power of given integer.

struct Opt {
    #[structopt(name = "Integer")]
    ///Integer
    a: usize,
    #[structopt(name = "Exponent")]
    ///Exponent
    x: u32,
}

fn power(a: usize, x: u32) -> Result<usize, &'static str> {
    match a.checked_pow(x) {
        Some(result) => Ok(result),
        None => Err("Overflow")
    }
}

fn main() {
    let args = Opt::from_args();
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
