use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Range")]
#[structopt(global_setting = structopt::clap::AppSettings::AllowNegativeNumbers)]

struct Args {
    #[structopt(name = "numbers")]
    numbers: Vec<i32>,
}

fn smallest(arr: &Vec<i32>) -> String {
    if arr[0] < arr[1] && arr[1] < arr[2] {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

fn main() {
    let args = Args::from_args();
    println!("{}", smallest(&args.numbers));
}

#[test]
fn test() {
    assert_eq!("Yes", smallest(&vec![1, 2, 3]));
    assert_eq!("No", smallest(&vec![1, 4, 3]));
}