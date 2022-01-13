use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Two Word Name")]

struct Args {
    #[structopt(name = "name")]
    name: String,
}

fn two_name(name: &str) -> String {
    name.split(' ')
        .map(|x| x.chars().nth(0).unwrap().to_string().to_uppercase())
        .collect::<Vec<String>>()
        .join(".")
}

fn main() {
    let args = Args::from_args();
    println!("{}",two_name(&args.name));
}

#[test]
fn test() {
    assert_eq!("A.I", two_name("Aa Ii"));
    assert_eq!("A.I", two_name("aa ii"));
}