/**
 * https://leetcode.com/problems/roman-to-integer/description/
 */

pub struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let map = s
            .replace("IV", "IIII")
            .replace("IX", "VIIII")
            .replace("XL", "XXXX")
            .replace("XC", "LXXXX")
            .replace("CD", "CCCC")
            .replace("CM", "DCCCC");

        map.chars()
            .map(|c| match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::roman_to_int("I".to_string()));
        assert_eq!(5, Solution::roman_to_int("V".to_string()));
        assert_eq!(10, Solution::roman_to_int("X".to_string()));
        assert_eq!(50, Solution::roman_to_int("L".to_string()));
        assert_eq!(100, Solution::roman_to_int("C".to_string()));
        assert_eq!(500, Solution::roman_to_int("D".to_string()));
        assert_eq!(1000, Solution::roman_to_int("M".to_string()));
    }

    #[test]
    fn test_2() {
        assert_eq!(3, Solution::roman_to_int("III".to_string()));
        assert_eq!(58, Solution::roman_to_int("LVIII".to_string()));
        println!("{}", Solution::roman_to_int("MCMXCIV".to_string()));
        assert_eq!(1994, Solution::roman_to_int("MCMXCIV".to_string()));
    }
}
