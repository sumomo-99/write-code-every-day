use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Remove String Spaces")]

struct Args {
    #[structopt(name = "string")]
    string: String,
}

fn remove_spaces(s: String) -> String {
    s.replace(" ", "")
}

fn main() {
    let args = Args::from_args();
    println!("{}", remove_spaces(args.string));
}

#[test]
fn test() {
    assert_eq!("abcd", remove_spaces("a b c d".to_string()));
}