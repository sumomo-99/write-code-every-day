use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Summation")]
#[structopt(global_setting = structopt::clap::AppSettings::AllowNegativeNumbers)]

struct Args {
    #[structopt(name = "number")]
    number: i32,
}

fn summation(n: i32) -> i32 {
    (1..=n).sum()
}

fn main() {
    let args = Args::from_args();
    println!("{}", summation(args.number));
}

#[test]
fn test() {
    assert_eq!(55, summation(10));
}
