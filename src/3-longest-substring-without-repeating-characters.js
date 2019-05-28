/**
 * Given a string, find the length of the longest substring without repeating characters.
 */

/**
 * @param {string} s
 * @return {number}
 */
var lengthOfLongestSubstring = function(s) {
  const l = s.length;
  let ans = 0;
  const obj = {};
  for (let i = 0, j = 0; j < l; j++) {
    if (obj[s[j]]) {
      i = Math.max(obj[s[j]], i);
    }
    ans = Math.max(ans, j - i + 1);
    obj[s[j]] = j + 1;
  }
  return ans;
};

// faster way
var lengthOfLongestSubstring = function(str) {
  let left = 0;
  let longestLength = 0;
  
  for (let right = 0; right < str.length; right++) {
    let char = str[right];
    
    while (str.indexOf(char, left) !== right) {
      left ++;
    }
    longestLength = Math.max(longestLength, right - left + 1);
  }
  return longestLength; 
};