use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Rectangle")]

struct Rectangle {
    #[structopt(name = "length")]
    a: u32,
    #[structopt(name = "breadth")]
    b: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.a * self.b
    }

    fn perimeter(&self) -> u32 {
        2 * (self.a + self.b)
    }
}

fn main() {
    let rectangle = Rectangle::from_args();
    println!("{} {}", rectangle.area(), rectangle.perimeter());
}

#[test]
fn test() {
    let t = Rectangle { a: 3, b: 5 };
    assert_eq!(t.area(), 15);
    assert_eq!(t.perimeter(), 16);
}
