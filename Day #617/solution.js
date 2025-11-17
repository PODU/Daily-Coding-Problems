// Roman numeral -> decimal. Single left-to-right pass; subtract when a smaller
// value precedes a larger one. Time O(n), Space O(1).
function romanToInt(s) {
  const val = { M: 1000, D: 500, C: 100, L: 50, X: 10, V: 5, I: 1 };
  let total = 0;
  for (let i = 0; i < s.length; i++) {
    const v = val[s[i]];
    if (i + 1 < s.length && val[s[i + 1]] > v) total -= v;
    else total += v;
  }
  return total;
}

console.log(romanToInt("XIV")); // 14
