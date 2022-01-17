use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Centur from Year")]

struct Args {
    #[structopt(name = "year")]
    year: u32,
}

fn century(year: u32) -> u32 {
    1 + (year -1) / 100
}

fn main() {
    let args = Args::from_args();
    println!("{}", century(args.year));
}

#[test]
fn test() {
    assert_eq!(1, century(90));
    assert_eq!(20, century(2000));
    assert_eq!(21, century(2021));
}