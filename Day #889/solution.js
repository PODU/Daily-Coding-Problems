// Run-length encoding/decoding. Single pass over the string.
// Time: O(n) encode/decode, Space: O(n) for output.
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
  const n = s.length;
  for (let i = 0; i < n; ) {
    let cnt = 0;
    while (i < n && s[i] >= "0" && s[i] <= "9") { cnt = cnt * 10 + (s.charCodeAt(i) - 48); i++; }
    const c = s[i++];
    out += c.repeat(cnt);
  }
  return out;
}

const input = "AAAABBBCCDAA";
const enc = encode(input);
console.log(enc);
console.log(decode(enc));
