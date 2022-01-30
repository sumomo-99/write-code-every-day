use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Reversed Numbers")]
struct Args {
    #[structopt(name = "numbers")]
    numbers: Vec<i32>,
}

fn reverse_numbers(numbers: &[i32]) -> String {
    let mut rev = numbers.to_owned();
    rev.reverse();
    rev.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    let args = Args::from_args();
    println!("{}", reverse_numbers(&args.numbers));
}

#[test]
fn test() {
    let vec = vec![1, 2, 3, 4];
    assert_eq!("4 3 2 1", reverse_numbers(&vec));
}
