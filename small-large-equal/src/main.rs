use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Small Large Equal")]
#[structopt(global_setting = structopt::clap::AppSettings::AllowNegativeNumbers)]

struct Args {
    #[structopt(name = "number-a")]
    a: isize,
    #[structopt(name = "number-b")]
    b: isize,
}

fn compare(a: isize, b: isize) -> String {
    if a < b {
        "a < b".to_string()
    } else if a > b {
        "a > b".to_string()
    } else {
        "a == b".to_string()
    }
}

fn main() {
    let args = Args::from_args();
    println!("{}", compare(args.a, args.b));
}

#[test]
fn test() {
    assert_eq!("a < b", compare(1, 2));
    assert_eq!("a > b", compare(1, -2));
    assert_eq!("a == b", compare(1, 1));
}