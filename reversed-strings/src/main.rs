use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Reversed Strings")]

struct Args {
    #[structopt(name = "strings")]
    s: String,
}

fn reversed(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let args = Args::from_args();
    println!("{}", reversed(&args.s));
}

#[test]
fn test() {
    assert_eq!("dlrow", reversed("world"));
}
