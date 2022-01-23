use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "A/B Problem")]

struct Args {
    #[structopt(name = "a")]
    a: i32,
    #[structopt(name = "b")]
    b: i32,
}

fn a_divide_b(a: i32, b: i32) -> (i32, i32, f32) {
    let d = (a/b, a%b, a as f32/b as f32);
    d
}

fn main() {
    let args = Args::from_args();
    println!("{:?}", a_divide_b(args.a, args.b));
}

#[test]
fn test() {
    assert_eq!((1, 1,1.50000), a_divide_b(3, 2));
}