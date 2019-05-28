/**
 * Given an array of integers, return indices of the two numbers such that they add up to a specific target.

 * You may assume that each input would have exactly one solution, and you may not use the same element twice.

 * Example:

 * Given nums = [2, 7, 11, 15], target = 9,

 * Because nums[0] + nums[1] = 2 + 7 = 9,
 * return [0, 1].
 */

/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[]}
 */
var twoSum = function(nums, target) {
  var l = nums.length;
  var i, j, iVal;
  for (i = 0; i < l - 1; ++i) {
    iVal = nums[i];
    for (j = i + 1; j < l; ++j) {
      if (iVal + nums[j] === target) {
        return [i, j];
      }
    }
  }
}

/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[]}
 */
var twoSum = function(nums, target) {
  var obj = {};
  var i, l = nums.length;
  for (i = 0; i < l; ++i) {
    var n = nums[i],
        del = target - nums[i];
    if (obj[del] !== undefined) {
      return [obj[del], i];
    }
    obj[n] = i
  }
};