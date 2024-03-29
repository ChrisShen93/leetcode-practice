/**
 * [3] Longest Substring Without Repeating Characters
 *
 * Given a string s, find the length of the longest substring without repeating characters.
 *
 * Example:
 *
 *
 * Input: s = "abcabcbb"
 * Output: 3
 * Explanation: The answer is "abc", with the length of 3.
 *
 */
use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut hash = HashMap::new();
        let mut start = 0;

        let mut max = 0;

        for (index, char) in s.chars().enumerate() {
            start = start.max(hash.insert(char, index as i32).unwrap_or(-1) + 1);
            max = max.max(index as i32 - start + 1);
        }

        max as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("abcabcbb")),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring(String::from("bbbbb")),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring(String::from("pwwkew")),
            3
        );
        assert_eq!(Solution::length_of_longest_substring(String::from(" ")), 1);
    }
}

