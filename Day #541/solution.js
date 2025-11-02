// Day 541: Run-length encoding/decoding. Scan runs to build count+char; parse digits to expand.
// Time O(n) encode, O(output) decode. Space O(n).

function encode(s) {
  let out = '';
  let i = 0;
  const n = s.length;
  while (i < n) {
    let j = i;
    while (j < n && s[j] === s[i]) j++;
    out += (j - i) + s[i];
    i = j;
  }
  return out;
}

function decode(s) {
  let out = '';
  let count = 0;
  for (const c of s) {
    if (c >= '0' && c <= '9') count = count * 10 + (c.charCodeAt(0) - 48);
    else { out += c.repeat(count); count = 0; }
  }
  return out;
}

const original = 'AAAABBBCCDAA';
const enc = encode(original);
console.log(enc);                             // 4A3B2C1D2A
console.log(decode(enc) === original);
