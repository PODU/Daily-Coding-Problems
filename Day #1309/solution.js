// Soundex phonetic encoding: keep first letter, map consonants to digits,
// collapse same-code runs, drop vowels, pad/truncate to 3 digits. O(n) time.
'use strict';

const CODES = {};
[["bfpv", 1], ["cgjkqsxz", 2], ["dt", 3], ["l", 4], ["mn", 5], ["r", 6]]
  .forEach(([letters, d]) => { for (const ch of letters) CODES[ch] = d; });

function code(c) { return CODES[c.toLowerCase()] || 0; }

function soundex(name) {
  if (!name) return "";
  let out = name[0].toUpperCase();
  let prev = code(name[0]);
  for (let i = 1; i < name.length && out.length < 4; i++) {
    const lc = name[i].toLowerCase();
    const c = code(lc);
    if (c !== 0 && c !== prev) out += String(c);
    if (lc === 'h' || lc === 'w') continue;
    prev = c;
  }
  while (out.length < 4) out += '0';
  return out.slice(0, 4);
}

console.log(soundex("Jackson")); // J250
console.log(soundex("Jaxen"));   // J250
