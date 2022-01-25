use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Min Max Sum")]
#[structopt(global_setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
struct Args {
    #[structopt(name = "numbers")]
    numbers: Vec<i32>,
}

fn calculator(numbers: &Vec<i32>) -> (i32, i32, i32) {
    let min = *numbers.iter().min().unwrap();
    let max = *numbers.iter().max().unwrap();
    let sum = numbers.iter().sum::<i32>();
    (min, max, sum)
}

fn main() {
    let args = Args::from_args();
    let (min, max, sum) = calculator(&args.numbers);
    println!("{} {} {}", min, max, sum);
}

#[test]
fn test() {
    let v = vec!(10, 1, 5, 4, 17);
    assert_eq!((1, 17, 37), calculator(&v));
}