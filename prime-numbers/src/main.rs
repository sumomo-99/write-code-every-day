use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Prime Numbers")]
struct Args {
    #[structopt(name = "x")]
    x: u32,
}

fn is_prime(x: u32) -> bool {
    if x == 2 {
        return true;
    };

    if x < 2 || x % 2 == 0 {
        return false;
    };

    let mut i = 3;
    let x_sqrt = (x as f64).sqrt();
    while i as f64 <= x_sqrt {
        if x % i == 0 {
            return false;
        }
        i += 2;
    }

    true
}

fn main() {
    let args = Args::from_args();
    println!("{}", is_prime(args.x));
}

#[test]
fn test() {
    assert_eq!(false, is_prime(1));
    assert_eq!(true, is_prime(2));
    assert_eq!(true, is_prime(3));
    assert_eq!(false, is_prime(4));
}
