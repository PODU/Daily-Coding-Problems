// Day 1294: Run-length encoding and decoding for alphabetic strings.
// Single linear scan for each direction. O(n) time, O(n) space.
function encode(s) {
  let out = "";
  const n = s.length;
  for (let i = 0; i < n; ) {
    let j = i;
    while (j < n && s[j] === s[i]) j++;
    out += (j - i) + s[i];
    i = j;
  }
  return out;
}

function decode(s) {
  let out = "";
  let count = 0;
  for (const c of s) {
    if (c >= "0" && c <= "9") count = count * 10 + (c.charCodeAt(0) - 48);
    else { out += c.repeat(count); count = 0; }
  }
  return out;
}

const e = encode("AAAABBBCCDAA");
console.log(e);          // 4A3B2C1D2A
console.log(decode(e));  // AAAABBBCCDAA
