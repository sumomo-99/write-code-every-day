use clap::Parser;

#[derive(Parser)]
///長方形の面積と周囲の長さを出力する。
struct Rectangle {
    ///長方形の長さ
    length: u32,
    ///長方形の幅
    breadth: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.breadth
    }

    fn perimeter(&self) -> u32 {
        2 * (self.length + self.breadth)
    }
}

fn main() {
    let rectangle = Rectangle::parse();
    println!("長さ: {}, 幅: {}, 面積: {}, 周囲: {}",
        rectangle.length,
        rectangle.breadth,
        rectangle.area(),
        rectangle.perimeter()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn area_test() {
        let t = Rectangle { length: 3, breadth: 5 };
        assert_eq!(t.area(), 15);
    }
    #[test]
    fn perimeter_test() {
        let t = Rectangle { length: 3, breadth: 5 };
        assert_eq!(t.perimeter(), 16);
    }

}
