use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Insert Sort")]
struct Args {
    #[structopt(name = "numbers")]
    numbers: Vec<i32>,
}

fn insert_sort(numbers: &[i32]) -> Vec<i32> {
    let mut vec = numbers.to_owned();
    for i in 1..vec.len() {
        let mut j = i;
        while j > 0 && vec[j - 1] > vec[j] {
            vec.swap(j - 1, j);
            match j.checked_sub(1) {
                Some(v) => {
                    j = v;
                }
                None => break,
            };
        }
    }
    vec
}

fn main() {
    let args = Args::from_args();
    println!("{:?}", insert_sort(&args.numbers));
}

#[test]
fn test() {
    let vec = vec![5, 2, 4, 6, 1, 3];
    assert_eq!(vec!(1, 2, 3, 4, 5, 6), insert_sort(&vec));
}
