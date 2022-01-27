use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Print Framw")]
struct Args {
    #[structopt(name = "height")]
    height: u32,
    #[structopt(name = "width")]
    width: u32,
}

fn rectangle_string(height: u32, width: u32) -> String {
    let mut rectagle = String::new();

    for i in 0..height {
        for j in 0..width {
            if i == 0 || i == height - 1 || j == 0 ||  j == width -1 {
                rectagle += "#";
            } else {
                rectagle += ".";
            }
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
    assert_eq!("####\n#..#\n####\n", rectangle_string(3, 4));
}