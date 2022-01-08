use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "String Repeat")]

struct Args {
    #[structopt(name = "word")]
    word: String,
    #[structopt(name = "counter")]
    counter: usize,
}

fn string_repeat(s: &str, counter: usize) -> String {
    s.repeat(counter)
}

fn main() {
    let args = Args::from_args();
    println!("{}", string_repeat(&args.word, args.counter));
}

#[test]
fn test() {
    assert_eq!("abcabc", string_repeat("abc", 2));
}
