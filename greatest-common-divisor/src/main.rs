use std::mem::swap;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Greatest Common Divisor")]
struct Args {
    #[structopt(name = "x")]
    x: i32,
    #[structopt(name = "y")]
    y: i32,
}

fn gcd(mut x: i32, mut y: i32) -> i32 {
    if x < y {
        swap(&mut x, &mut y);
    }

    while y > 0 {
        let r = x % y;
        x = y;
        y = r;
    }

    x
}

fn main() {
    let args = Args::from_args();
    println!("{}", gcd(args.x, args.y));
}

#[test]
fn test() {
    assert_eq!(2, gcd(74, 54));
}
