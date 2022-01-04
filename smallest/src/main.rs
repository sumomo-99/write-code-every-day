use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Smallest")]
#[structopt(global_setting = structopt::clap::AppSettings::AllowNegativeNumbers)]

struct Args {
    #[structopt(name = "numbers")]
    numbers: Vec<i32>,
}

fn smallest(arr: &Vec<i32>) -> i32 {
    *arr.iter().min().unwrap()
}

fn main() {
    let args = Args::from_args();
    println!("{}", smallest(&args.numbers));
}

#[test]
fn test() {
    assert_eq!(1, smallest(&vec![1, 2, 3]));
}