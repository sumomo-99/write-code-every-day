use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Print Many Hello World")]

struct Args {
    #[structopt(name = "num")]
    num: u32,
}

fn hello(num: &u32) -> String {
    let mut n = 0;
    let mut s = String::new();

    while n < *num {
        s += "Hello World\n";
        n += 1;
    }

    s
}

fn main() {
    let args = Args::from_args();
    println!("{}", hello(&args.num));
}

#[test]
fn test() {
    assert_eq!("Hello World\nHello World\n", hello(&2));
}
