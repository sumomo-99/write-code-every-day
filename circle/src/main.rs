use structopt::StructOpt;

const PI: f32 = 3.141592653589;

#[derive(StructOpt)]
#[structopt(name = "Circle")]
struct Args {
    #[structopt(name = "radius")]
    r: f32,
}

fn area(r: &f32) -> f32 {
    r * r * PI
}

fn circumference(r: &f32) -> f32 {
    2.0 * r * PI
}

fn main() {
    let args = Args::from_args();
    println!("{} {}", area(&args.r), circumference(&args.r), );
}

#[test]
fn test() {
    assert_eq!(28.274334, area(&3.0));
    assert_eq!(18.849556, circumference(&3.0));
}