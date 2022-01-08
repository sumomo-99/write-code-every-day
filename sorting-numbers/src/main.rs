use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Sorting Numbers")]

struct Args {
    #[structopt(name = "numbers")]
    numbers: Vec<i32>,
}

fn sorting_numbers(numbers: &[i32]) -> String {
    let mut vec = numbers.to_owned();
    vec.sort_unstable();
    let vec: Vec<String> = vec.iter().map(|x| x.to_string()).collect();
    vec.join(" ")
}

fn main() {
    let args = Args::from_args();
    println!("{}", sorting_numbers(&args.numbers));
}

#[test]
fn test() {
    assert_eq!("1 3 8", sorting_numbers(&vec![3, 8, 1]));
}
