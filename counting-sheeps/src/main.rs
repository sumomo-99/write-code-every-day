use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Counting Sheeps")]

struct Args {
    #[structopt(name = "sheeps")]
    sheeps: Vec<bool>,
}

fn counting_sheeps(sheeps: &Vec<bool>) -> u32 {
    sheeps.iter().filter(|&&x| x).count() as u32
}

fn main() {
    let args = Args::from_args();
    println!("{}", counting_sheeps(&args.sheeps));
}

#[test]
fn test() {
    assert_eq!(0, counting_sheeps(&vec![]));
    assert_eq!(1, counting_sheeps(&vec![true]));
    assert_eq!(0, counting_sheeps(&vec![false]));
    assert_eq!(1, counting_sheeps(&vec![true, false]));
    assert_eq!(2, counting_sheeps(&vec![true, true]));
    assert_eq!(0, counting_sheeps(&vec![false, false]));
}