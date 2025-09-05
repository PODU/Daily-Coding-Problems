// Day 221: nth "sevenish" number (sum of distinct powers of 7).
// Approach: bits of n select which powers of 7 to include (bijection with binary). Time O(log n), Space O(1).
function sevenish(n) {
  let result = 0, power = 1; // 7^0
  while (n > 0) {
    if (n & 1) result += power;
    power *= 7;
    n = Math.floor(n / 2);
  }
  return result;
}

console.log([1, 2, 3, 4, 5].map(sevenish).join(" ")); // 1 7 8 49 50
console.log(sevenish(4)); // 49
