// Day 123: Validate whether a string is a number (int/real/scientific).
// Single linear scan state machine. O(n) time, O(1) space.
function isNumber(s) {
  let i = 0;
  const n = s.length;
  if (n === 0) return false;
  if (s[i] === "+" || s[i] === "-") i++;
  let digits = 0, dots = 0;
  const isDigit = (c) => c >= "0" && c <= "9";
  while (i < n && (isDigit(s[i]) || s[i] === ".")) {
    if (s[i] === ".") dots++;
    else digits++;
    i++;
  }
  if (dots > 1 || digits === 0) return false;
  if (i < n && (s[i] === "e" || s[i] === "E")) {
    i++;
    if (i < n && (s[i] === "+" || s[i] === "-")) i++;
    let expd = 0;
    while (i < n && isDigit(s[i])) { expd++; i++; }
    if (expd === 0) return false;
  }
  return i === n;
}

const tests = ["10", "-10", "10.1", "-10.1", "1e5", "a", "x 1", "a -2", "-"];
for (const t of tests) console.log('"' + t + '" -> ' + isNumber(t));
