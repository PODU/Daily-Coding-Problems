// Day 1282: n-th positive integer whose digits sum to 10.
// Such numbers are ~ every 9th integer; iterate counting matches. Time O(answer/9), Space O(1).
function digitSum(x) {
  let s = 0;
  while (x > 0) { s += x % 10; x = Math.floor(x / 10); }
  return s;
}

function nthPerfect(n) {
  let x = 0, count = 0;
  while (count < n) {
    x++;
    if (digitSum(x) === 10) count++;
  }
  return x;
}

console.log(nthPerfect(1)); // 19
console.log(nthPerfect(2)); // 28
