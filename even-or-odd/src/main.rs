use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "EvenOrOdd")]

struct Number {
    #[structopt(name = "number")]
    number: u32,
}

impl Number {
    fn check(&self) -> &str {
        match self.number % 2 {
            0 => "Even",
            _ => "Odd",
        }
    }
}

fn main() {
    let number = Number::from_args();
    println!("{}", number.check());
}

#[test]
fn test() {
    let num = Number { number: 2 };
    assert_eq!(num.check(), "Even");
    let num = Number { number: 3 };
    assert_eq!(num.check(), "Odd");
}
