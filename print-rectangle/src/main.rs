use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Print Rectangle")]
struct Args {
    #[structopt(name = "height")]
    height: u32,
    #[structopt(name = "width")]
    width: u32,
}

fn rectangle_string(height: u32, width: u32) -> String {
    let mut rectagle = String::new();

    for _i in 0..height {
        for _j in 0..width {
            rectagle += "#";
        }
        rectagle += "\n";
    }

    rectagle
}

fn main() {
    let args = Args::from_args();
    println!("{}", rectangle_string(args.height, args.width));
}

#[test]
fn test() {
    assert_eq!("###\n###\n", rectangle_string(2, 3));
}