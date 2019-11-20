/**
 * Say you have an array for which the ith element is the price of a given stock on day i.
 * Design an algorithm to find the maximum profit. You may complete as many transactions as you like (i.e., buy one and sell one share of the stock multiple times).
 * Note: You may not engage in multiple transactions at the same time (i.e., you must sell the stock before you buy again).
 *
 * console.log(maxProfit([7, 1, 5, 3, 6, 4]) === 7);
 * console.log(maxProfit([1, 2, 3, 4, 5]) === 4);
 * console.log(maxProfit([7, 6, 4, 3, 1]) === 0);
 *
 * console.log(maxProfit1([7, 1, 5, 3, 6, 4]) === 7);
 * console.log(maxProfit1([1, 2, 3, 4, 5]) === 4);
 * console.log(maxProfit1([7, 6, 4, 3, 1]) === 0);
 *
 * console.log(maxProfit1([2, 1, 2, 0, 1]) === 2);
 */

/**
 * @param {number[]} prices
 * @return {number}
 */
var maxProfit = function(prices) {
  if (!prices || !prices.length) return 0;

  let ret = 0;

  for (let i = 1; i < prices.length; i++) {
    if (prices[i - 1] < prices[i]) {
      ret += prices[i] - prices[i - 1];
    }
  }

  return ret;
};

var maxProfit1 = function(prices) {
  if (!prices || !prices.length) return 0;

  let ret = 0;
  let current = -Infinity;

  for (let i = 0; i < prices.length; i++) {
    const p = prices[i];
    if (current === -Infinity) {
      if (prices[i + 1] > p) {
        current = p;
      }
    } else {
      if (p > prices[i + 1]) {
        ret += p - current;
        current = -Infinity;
      }
    }
  }

  if (current !== -Infinity) {
    ret += prices[prices.length - 1] - current;
  }

  return ret;
}
