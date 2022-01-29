use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Print Chessboard")]
struct Args {
    #[structopt(name = "height")]
    height: u32,
    #[structopt(name = "width")]
    width: u32,
}

fn chessboard(height: u32, width: u32) -> String {
    let mut board = String::new();
    for h in 1..=height {
        for w in 1..=width {
            match h % 2 {
                0 => match w % 2 {
                    0 => board += "#",
                    _ => board += ".",
                },
                _ => match w % 2 {
                    0 => board += ".",
                    _ => board += "#",
                },
            }
        }
        board += "\n"
    }
    board
}

fn main() {
    let args = Args::from_args();
    println!("{}", chessboard(args.height, args.width));
}

#[test]
fn test() {
    assert_eq!("#.#.\n.#.#\n#.#.\n", chessboard(3, 4));
}
