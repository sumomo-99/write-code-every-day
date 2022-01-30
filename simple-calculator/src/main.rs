use structopt::StructOpt;
use rust_decimal::prelude::*;


#[derive(StructOpt)]
#[structopt(name = "Simple Calculator")]

struct Args {
    #[structopt(name = "a")]
    a: Decimal,
    #[structopt(name = "operator")]
    op: char,
    #[structopt(name = "b")]
    b: Decimal,
}

fn calculator(a: &Decimal, op: &char, b: &Decimal) -> String {
    match op {
       '+' => (a + b).to_string(),
       '-' => (a - b).to_string(),
       'x' => (a * b).to_string(),
       '/' => (a / b).to_string(),
       _ => "".to_string()
    }
}

fn main() {
    let args = Args::from_args();
    println!("{:.8}", calculator(&args.a, &args.op, &args.b));
}

#[test]
fn test() {
    assert_eq!("2.1", format!("{:.3}" ,calculator(&Decimal::new(11, 1), &'+', &Decimal::new(10, 1))));
    assert_eq!("0.1", format!("{:.3}" ,calculator(&Decimal::new(11, 1), &'-', &Decimal::new(10, 1))));
    assert_eq!("1.1", format!("{:.3}" ,calculator(&Decimal::new(11, 1), &'x', &Decimal::new(10, 1))));
    assert_eq!("1.1", format!("{:.3}" ,calculator(&Decimal::new(11, 1), &'/', &Decimal::new(10, 1))));
}