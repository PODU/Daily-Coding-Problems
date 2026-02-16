// Roman to decimal: add each value, subtract when a smaller numeral precedes a larger. Time O(n), Space O(1).
function romanToInt(s) {
  const v = { M: 1000, D: 500, C: 100, L: 50, X: 10, V: 5, I: 1 };
  let total = 0;
  for (let i = 0; i < s.length; i++) {
    if (i + 1 < s.length && v[s[i]] < v[s[i + 1]]) total -= v[s[i]];
    else total += v[s[i]];
  }
  return total;
}

console.log(romanToInt("XIV"));
