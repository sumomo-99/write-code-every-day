use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "x-cubic")]
///Calculates the cube of given integer.

struct Opt {
    #[structopt(name = "NUMBER")]
    ///Integer
    x: u32,
}

fn cubic(x: u32) -> u32 {
    x.pow(3)
}

fn main() {
    let state = Opt::from_args();
    println!("{}", cubic(state.x));
}

#[test]
fn test_cubic() {
    assert_eq!(8, cubic(2));
    assert_eq!(27, cubic(3));
}
