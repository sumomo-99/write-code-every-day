use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Sum Of Positive")]
#[structopt(global_setting = structopt::clap::AppSettings::AllowNegativeNumbers)]

struct Args {
    #[structopt(name = "numbers")]
    numbers: Vec<i32>,
}

fn sum(numbers: Vec<i32>) -> i32 {
    numbers.iter().filter(|x| x.is_positive()).sum()
}

fn main() {
    let args = Args::from_args();
    println!("{}", sum(args.numbers));
}

#[test]
fn test() {
    let v = vec![1, -4, 7, 12];
    assert_eq!(20, sum(v));
}
