use clap::Parser;
use rand::Rng;

#[derive(Parser)]
///サイコロの合計を表示します。
struct Args {
    ///サイコロを振る回数
    num: u32,
}

fn dice(num: u32) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut nums = Vec::new();
    for _ in 0..num {
        nums.push(rng.gen_range(1..=6));
    }
    nums
}

fn dice_sum(nums: Vec<u32>) -> u32 {
    nums.iter().sum()
}

fn main() {
    let args = Args::parse();
    let nums = dice(args.num);
    println!("{:?}", nums);
    println!("{}", dice_sum(nums));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dice() {
        let nums = dice(5);
        assert_eq!(5, nums.len());
    }
}