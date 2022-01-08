use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Square(n) Sum")]

struct Args {
    #[structopt(name = "length")]
    length: Vec<i32>,
}

fn square_sum(vec: Vec<i32>) -> i32 {
    vec.iter().map(|x| x * x).sum()
}

fn main() {
    let args = Args::from_args();
    println!("{}", square_sum(args.length));
}

#[test]
fn test() {
    assert_eq!(14, square_sum(vec![1, 2, 3]));
    assert_eq!(0, square_sum(vec![]));
}
