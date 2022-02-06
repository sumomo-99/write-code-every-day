use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Selection Sort")]
#[structopt(global_setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
struct Args {
    #[structopt(name = "numbers")]
    numbers: Vec<i32>,
}

fn selection_sort(numbers: Vec<i32>) -> Vec<i32> {
    let mut minj;
    let mut vec = numbers;

    for i in 0..vec.len() {
        minj = i;
        for j in i..vec.len() {
            if vec[j] < vec[minj] {
                minj = j
            }
        }
        vec.swap(i, minj);
    }

    vec
}

fn main() {
    let args = Args::from_args();
    println!("{:?}", selection_sort(args.numbers));
}

#[test]
fn test() {
    let v = vec![5, 3, 2, 4, 1];
    let a = vec![1, 2, 3, 4, 5];
    assert_eq!(a, selection_sort(v));
}
