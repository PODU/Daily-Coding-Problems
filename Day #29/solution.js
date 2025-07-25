// Run-length encoding/decoding in a single pass.
// Time: O(n), Space: O(n) for output.
function encode(s) {
  let res = "";
  const n = s.length;
  for (let i = 0; i < n; ) {
    let j = i;
    while (j < n && s[j] === s[i]) j++;
    res += (j - i) + s[i];
    i = j;
  }
  return res;
}

function decode(s) {
  let res = "";
  const n = s.length;
  for (let i = 0; i < n; ) {
    let count = 0;
    while (i < n && s[i] >= "0" && s[i] <= "9") {
      count = count * 10 + (s.charCodeAt(i) - 48);
      i++;
    }
    res += s[i].repeat(count);
    i++;
  }
  return res;
}

const input = "AAAABBBCCDAA";
const enc = encode(input);
console.log(enc);
console.log(decode(enc));
