/**
 * @param {number[]} nums1
 * @param {number[]} nums2
 * @return {number}
 */
var findMedianSortedArrays = function(nums1, nums2) {
  const arr = nums1.concat(nums2).sort(function (a, b) {
    return a - b;
  });
  const l = arr.length;
  if (l % 2) {
    return arr[(l - 1) / 2];
  } else {
    return (arr[l / 2 - 1] + arr[l / 2]) / 2;
  }
};

// a faster and the most smallest way
var findMedianSortedArrays = function (nums1, nums2) {
  const midIndex = (nums1.length + nums2.length) / 2;
  let p1 = 0;
  let p2 = 0;
  let median, last;
  for (let i = 0; i <= midIndex; ++i) {
    const n1 = nums1[p1] !== undefined ? nums1[p1] : Infinity;
    const n2 = nums2[p2] !== undefined ? nums2[p2] : Infinity;
    if (n1 >= n2) {
      last = median;
      median = n2;
      ++p2;
    } else {
      last = median;
      median = n1;
      ++p1;
    }
  }
  if ((nums1.length + nums2.length) % 2 === 0) {
    median = (median + last) / 2;
  }
  return median;
}