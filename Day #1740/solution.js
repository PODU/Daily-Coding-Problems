// Approach: deterministic single linear scan validating sign/digits/dot/exponent.
// Time O(n), Space O(1).
function isNumber(s) {
  const n = s.length;
  if (n === 0) return false;
  let i = 0;
  const isDigit = (c) => c >= '0' && c <= '9';
  if (s[i] === '+' || s[i] === '-') i++;
  let digits = false, dot = false;
  while (i < n && (isDigit(s[i]) || s[i] === '.')) {
    if (s[i] === '.') {
      if (dot) return false;
      dot = true;
    } else digits = true;
    i++;
  }
  if (!digits) return false;
  if (i < n && (s[i] === 'e' || s[i] === 'E')) {
    i++;
    if (i < n && (s[i] === '+' || s[i] === '-')) i++;
    let expDigits = false;
    while (i < n && isDigit(s[i])) { expDigits = true; i++; }
    if (!expDigits) return false;
  }
  return i === n;
}

const tests = ["10", "-10", "10.1", "-10.1", "1e5", "a", "x 1", "a -2", "-"];
for (const t of tests) console.log(isNumber(t) ? "true" : "false");
