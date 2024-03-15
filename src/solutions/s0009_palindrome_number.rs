/**
 * [9] Palindrome Number
 *
 * Given an integer x, return true if x is palindrome, and false otherwise.
 *
 * Example:
 *
 *
 * Input: x = 121
 * Output: true
 * Explanation: 121 reads as 121 from left to right and from right to left.
 */

pub struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let x = x.to_string();
        let n = x.len();
        let x = x.chars().collect::<Vec<char>>();
        let head = x[..n / 2].iter();
        let tail = x[n / 2..].iter().rev();
        head.zip(tail).all(|(a, b)| a == b)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::s0009_palindrome_number::Solution;

    #[test]
    fn test_9() {
        assert_eq!(Solution::is_palindrome(121), true);

        assert_eq!(Solution::is_palindrome(-121), false);

        assert_eq!(Solution::is_palindrome(10), false);
    }
}
