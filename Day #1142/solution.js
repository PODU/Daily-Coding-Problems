// Day 1142: nth sevenish number = sum of distinct powers of 7.
// Bits of n in binary select powers of 7 (bijection). Time O(log n), Space O(1).
function sevenish(n) {
  let result = 0, power = 1;
  while (n > 0) {
    if (n & 1) result += power;
    power *= 7;
    n >>= 1;
  }
  return result;
}

console.log([1, 2, 3, 4, 5].map(sevenish).join(" ")); // 1 7 8 49 50
