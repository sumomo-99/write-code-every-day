use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Check for Factor")]

struct Args {
    #[structopt(name = "base")]
    base: i32,
    #[structopt(name = "factor")]
    factor: i32,
}

fn check_factor(base: i32, factor: i32) -> bool {
    base % factor == 0
}

fn main() {
    let args = Args::from_args();
    println!("{}", check_factor(args.base, args.factor));
}

#[test]
fn test() {
    assert_eq!(true, check_factor(10, 5));
    assert_eq!(false, check_factor(10, 3));
}