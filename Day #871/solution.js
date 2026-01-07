// Soundex phonetic encoding: keep first letter, code rest, dedupe, pad to 3 digits.
// Time O(n), Space O(1) extra per name.

const CODE = {};
for (const [chs, d] of [["bfpv", 1], ["cgjkqsxz", 2], ["dt", 3], ["l", 4], ["mn", 5], ["r", 6]]) {
  for (const ch of chs) CODE[ch] = d;
}

function code(c) {
  return CODE[c.toLowerCase()] || 0;
}

function isAlpha(c) {
  return /[a-z]/i.test(c);
}

function soundex(name) {
  let i = 0;
  while (i < name.length && !isAlpha(name[i])) i++;
  if (i >= name.length) return "";
  const res = [name[i].toUpperCase()];
  let prev = code(name[i]);
  for (let j = i + 1; j < name.length && res.length < 4; j++) {
    const c = name[j].toLowerCase();
    if (!isAlpha(c)) continue;
    if (c === "h" || c === "w") continue;
    const d = code(c);
    if (d === 0) { prev = 0; continue; }
    if (d !== prev) res.push(String(d));
    prev = d;
  }
  while (res.length < 4) res.push("0");
  return res.slice(0, 4).join("");
}

console.log(soundex("Jackson"));
console.log(soundex("Jaxen"));
