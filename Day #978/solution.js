// Valid number check via single-pass finite-state parser.
// Grammar: [sign] digits [. digits] | [sign] [digits] . digits, then optional (e/E [sign] digits).
// Time: O(n); Space: O(1).
"use strict";

function isDigit(c) {
  return c >= "0" && c <= "9";
}

function isNumber(s) {
  let i = 0;
  const n = s.length;
  if (n === 0) return false;
  if (s[i] === "+" || s[i] === "-") i++;

  let digitsBefore = false, digitsAfter = false;
  while (i < n && isDigit(s[i])) { i++; digitsBefore = true; }
  if (i < n && s[i] === ".") {
    i++;
    while (i < n && isDigit(s[i])) { i++; digitsAfter = true; }
  }
  if (!digitsBefore && !digitsAfter) return false;   // mantissa needs a digit

  if (i < n && (s[i] === "e" || s[i] === "E")) {
    i++;
    if (i < n && (s[i] === "+" || s[i] === "-")) i++;
    let expDigits = false;
    while (i < n && isDigit(s[i])) { i++; expDigits = true; }
    if (!expDigits) return false;                    // exponent needs a digit
  }
  return i === n;                                     // no trailing junk
}

const valid = ["10", "-10", "10.1", "-10.1", "1e5"];
const invalid = ["a", "x 1", "a -2", "-", "", " "];
for (const s of [...valid, ...invalid]) {
  console.log(`"${s}" -> ${isNumber(s)}`);
}
