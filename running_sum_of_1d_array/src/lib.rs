pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut sum = 0;
    let mut ret: Vec<i32> = Vec::new();

    for num in nums.iter() {
        sum += num;
        ret.push(sum);
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_running_sum_1() {
        let nums = vec!(1, 2, 3, 4);
        assert_eq!(vec!(1, 3, 6, 10), running_sum(nums));
    }
    #[test]
    fn test_running_sum_2() {
        let nums = vec!(1, 1, 1, 1, 1);
        assert_eq!(vec!(1, 2, 3, 4, 5), running_sum(nums));
    }
    #[test]
    fn test_running_sum_3() {
        let nums = vec!(3, 1, 2, 10, 1);
        assert_eq!(vec!(3, 4, 6, 16, 17), running_sum(nums));
    }
}