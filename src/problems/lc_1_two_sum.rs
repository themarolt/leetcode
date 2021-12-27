use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for (i, val) in nums.iter().enumerate() {
            let second_val: i32 = target - val;

            match map.get(&second_val) {
                Some(j) => return vec![*j, i as i32],
                None => {
                    map.insert(*val, i as i32);
                }
            }
        }
        return vec![];
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::lc_1_two_sum::Solution;

    #[test]
    fn example1() {
        let input = vec![2, 7, 11, 15];
        let target = 9;

        assert_eq!(Solution::two_sum(input, target), vec![0, 1]);
    }

    #[test]
    fn example2() {
        let input = vec![3, 2, 4];
        let target = 6;

        assert_eq!(Solution::two_sum(input, target), vec![1, 2]);
    }

    #[test]
    fn example3() {
        let input = vec![3, 3];
        let target = 6;

        assert_eq!(Solution::two_sum(input, target), vec![0, 1]);
    }
}
