use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Rectangle")]
///Calculate area and perimeter of rectangle.

struct Rectangle {
    #[structopt(name = "length")]
    ///Length of rectangle.
    l: u32,
    #[structopt(name = "breadth")]
    ///Breadth of rectangle.
    b: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.l * self.b
    }

    fn perimeter(&self) -> u32 {
        2 * (self.l + self.b)
    }
}

fn main() {
    let rectangle = Rectangle::from_args();
    println!("Length: {}", rectangle.l);
    println!("Breadth: {}", rectangle.b);
    println!("Area: {}", rectangle.area());
    println!("Perimeter: {}", rectangle.perimeter());
}

#[test]
fn test() {
    let t = Rectangle { l: 3, b: 5 };
    assert_eq!(t.area(), 15);
    assert_eq!(t.perimeter(), 16);
}
