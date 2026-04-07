// Day 1321: Roman numeral -> decimal.
// Approach: single left-to-right pass; subtract if a smaller value precedes a larger one. O(n) time, O(1) space.

function romanToInt(s) {
  const v = { M: 1000, D: 500, C: 100, L: 50, X: 10, V: 5, I: 1 };
  let total = 0;
  for (let i = 0; i < s.length; i++) {
    const cur = v[s[i]];
    if (i + 1 < s.length && cur < v[s[i + 1]]) total -= cur;
    else total += cur;
  }
  return total;
}

console.log(romanToInt("XIV")); // 14
