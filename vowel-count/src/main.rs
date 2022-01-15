use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Vowel Count")]

struct Args {
    #[structopt(name = "string")]
    string: String,
}

fn vowel_count(string: &str) -> usize {
    string
        .chars()
        .filter(|&c| "aiueo".contains(c))
        .count()
}

fn main() {
    let args = Args::from_args();
    println!("{}", vowel_count(&args.string));
}

#[test]
fn test() {
    assert_eq!(3, vowel_count("abcdefghi"));
}