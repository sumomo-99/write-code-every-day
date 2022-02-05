use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Bubble Sort")]
#[structopt(global_setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
struct Args {
    #[structopt(name = "numbers")]
    numbers: Vec<i32>,
}

fn bubble_sort(numbers: Vec<i32>) -> Vec<i32> {
    let mut flag = true;
    let mut vec = numbers;

    while flag {
        flag = false;
        for j in (1..vec.len()).rev() {
            if vec[j] < vec[j - 1] {
                vec.swap(j - 1, j);
                flag = true;
            }
        }
    }

    vec
}

fn main() {
    let args = Args::from_args();
    println!("{:?}", bubble_sort(args.numbers));
}

#[test]
fn test() {
    let v = vec![5, 3, 2, 4, 1];
    let a = vec![1, 2, 3, 4, 5];
    assert_eq!(a, bubble_sort(v));
}
