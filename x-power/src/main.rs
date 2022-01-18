use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "x-power")]
///Calculates Xth power of given integer.

struct Opt {
    #[structopt(name = "Integer")]
    ///Integer
    a: u32,
    #[structopt(name = "Power")]
    ///Power
    x: u32,
}

fn power(a: u32, x: u32) -> u32 {
    a.pow(x)
}

fn main() {
    let args = Opt::from_args();
    println!("{}", power(args.a, args.x));
}

#[test]
fn test_power() {
    assert_eq!(9, power(3, 2));
    assert_eq!(27, power(3, 3));
}
