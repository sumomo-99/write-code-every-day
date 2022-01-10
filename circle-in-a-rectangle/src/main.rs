use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Circle in a Rectangle")]

struct Args {
    #[structopt(name = "width")]
    width: u32,
    #[structopt(name = "hight")]
    height: u32,
    #[structopt(name = "x")]
    x: u32,
    #[structopt(name = "y")]
    y: u32,
    #[structopt(name = "r")]
    r: u32,
}

fn check(args: &Args) -> String {
    if args.x < args.r ||
       args.x + args.r > args.width ||
       args.y < args.r ||
       args.y + args.r > args.height {
           "No".to_string()
       } else {
           "Yes".to_string()
       }
}

fn main() {
    let args = Args::from_args();
    println!("{}",check(&args));
}

#[test]
fn test() {
    let args = Args {
        width: 5,
        height: 4,
        x: 2,
        y: 2,
        r: 1
    };
    assert_eq!("Yes", check(&args));
    let args = Args {
        width: 5,
        height: 4,
        x: 2,
        y: 4,
        r: 1
    };
    assert_eq!("No", check(&args));
}