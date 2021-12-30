use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Remove First and Last Characters")]

struct Args {
    #[structopt(name = "word")]
    word: String,
}

fn remove_char(s: &str) -> String {
    s[1..(s.len() - 1)].to_string()
}

fn main() {
    let args = Args::from_args();
    println!("{}", remove_char(&args.word));
}

#[test]
fn test() {
    assert_eq!("bc", remove_char("abcd"));
    assert_eq!("", remove_char("ab"));
}