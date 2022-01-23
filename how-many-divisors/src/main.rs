use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "How Many Divisors")]

struct Args {
    #[structopt(name = "start")]
    a: i32,
    #[structopt(name = "end")]
    b: i32,
    #[structopt(name = "base")]
    c: i32,
}

fn count_divisors(a: i32, b: i32, c: i32) -> i32 {
    let mut count = 0;
    for i in a..=b {
        match c % i {
            0 => count += 1,
            _ => ()
        }
    }
    count
}

fn main() {
    let args = Args::from_args();
    println!("{}", count_divisors(args.a, args.b, args.c));
}

#[test]
fn test() {
    assert_eq!(3, count_divisors(5, 14, 80));
    assert_eq!(5, count_divisors(5, 20, 80));
}