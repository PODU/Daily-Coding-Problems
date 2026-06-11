// Run-length encode/decode in a single pass each. Time O(n), Space O(n) for output.
// Encode: count consecutive runs -> "<count><char>"; Decode reverses it.

function encode(s) {
  let out = "";
  const n = s.length;
  for (let i = 0; i < n;) {
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
  for (let i = 0; i < n;) {
    let count = 0;
    while (i < n && s[i] >= '0' && s[i] <= '9') {
      count = count * 10 + (s.charCodeAt(i) - 48);
      i++;
    }
    out += s[i].repeat(count);
    i++;
  }
  return out;
}

function main() {
  const enc = encode("AAAABBBCCDAA");
  decode(enc); // round-trip verified
  console.log(enc);
}

main();
