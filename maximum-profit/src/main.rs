use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Maximum Profit")]
#[structopt(global_setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
struct Args {
    #[structopt(name = "numbers")]
    numbers: Vec<i32>,
}

fn max_profit(numbers: Vec<i32>) -> i32 {
    let mut minv = numbers[0];
    let mut maxv = numbers[1] - numbers[0];

    for num in numbers.iter().skip(1) {
        if (*num - minv) > maxv {
            maxv = *num - minv;
        };
        if *num < minv {
            minv = *num;
        };
    }

    maxv
}

fn main() {
    let args = Args::from_args();
    println!("{}", max_profit(args.numbers));
}

#[test]
fn test() {
    let v = vec![5, 3, 1, 3, 4, 3];
    assert_eq!(3, max_profit(v));
    let v = vec![4, 3, 2];
    assert_eq!(-1, max_profit(v));
}
