use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "EvenOrOdd")]

struct Second {
    #[structopt(name = "second")]
    input: u32,
}

impl Second {
    fn format(&self) -> String {
        match self.input {
            0..=86400 => {
                let tap = self.calc();
                format!("{}:{}:{}", tap.0, tap.1, tap.2)
            },
            _ => "enter a number between 0 and 86400.".to_string(),
        }
    }

    fn calc(&self) -> (u32, u32, u32){
        let mut num = self.input;
        let mut tap = (0, 0, 0);

        if num >= 3600 {
            tap.0 = num / 3600;
            num = num % 3600;
        }
        if num >= 60 {
            tap.1 = num / 60;
            num = num % 60;
        }
        tap.2 = num;
        tap
    }
}

fn main() {
    let number = Second::from_args();
    println!("{}", number.format());
}

#[test]
fn test() {
    let num = Second {
        input: 46979,
    };
    assert_eq!(num.format(), "13:2:59");
}
