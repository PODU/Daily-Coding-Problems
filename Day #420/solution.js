// Day 420: n-th positive integer whose digits sum to exactly 10.
// Iterate integers, count those with digit sum 10. Time: O(answer), Space: O(1).
function digitSum(x) {
  let s = 0;
  while (x > 0) {
    s += x % 10;
    x = Math.floor(x / 10);
  }
  return s;
}

function nthPerfect(n) {
  let count = 0;
  let x = 0;
  while (count < n) {
    x++;
    if (digitSum(x) === 10) count++;
  }
  return x;
}

console.log(nthPerfect(1)); // 19
console.log(nthPerfect(2)); // 28
