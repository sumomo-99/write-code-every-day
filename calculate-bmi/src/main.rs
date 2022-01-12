use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Calculate BMI")]

struct Args {
    #[structopt(name = "weight")]
    weight: f32,
    #[structopt(name = "hight")]
    height: f32,
}

fn calculate_bmi(weight: &f32,  height: &f32) -> String {
    let bmi = weight / height.powi(2);

    match bmi {
        bmi if bmi <= 18.5 => "Underweight".to_string(),
        bmi if bmi <= 25.0 => "Normal".to_string(),
        bmi if bmi <= 30.0 => "Overweight".to_string(),
        _ => "Obese".to_string(),
    }
}

fn main() {
    let args = Args::from_args();
    println!("{}",calculate_bmi(&args.weight, &args.height));
}

#[test]
fn test() {
    assert_eq!("Underweight", calculate_bmi(&50.0, &1.80));
    assert_eq!("Normal", calculate_bmi(&80.0, &1.80));
    assert_eq!("Overweight", calculate_bmi(&90.0, &1.80));
    assert_eq!("Obese", calculate_bmi(&110.0, &1.80));
}