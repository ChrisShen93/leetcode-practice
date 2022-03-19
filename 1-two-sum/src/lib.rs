use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        // this will work
        // let mut hash = HashMap::new();
        // for (index, item) in nums.iter().enumerate() {
        //     if hash.contains_key(item) {
        //         return vec![hash[item], index as i32];
        //     } else {
        //         hash.insert(target - item, index as i32);
        //     }
        // }
        // unreachable!();

        // better way
        let mut i = 0;
        let mut map = HashMap::new();

        while i < nums.len() {
            match map.get(&nums[i]) {
                Some(val) => return vec!(*val as i32, i as i32),
                None => map.insert(target - nums[i], i)
            };
            i += 1
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
    }
}
