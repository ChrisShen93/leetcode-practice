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
