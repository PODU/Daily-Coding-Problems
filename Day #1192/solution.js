// Step word: dict word of length len(input)+1 that contains all input letters plus exactly one extra,
// AND is a genuine anagram (rearrangement), not the input with a letter merely appended (no input prefix).
// Compare 26-letter frequency counts: every count >= input's and total diff == 1. O(D * (L + 26)).
"use strict";

function counts(s) {
  const c = new Array(26).fill(0);
  for (const ch of s) c[ch.charCodeAt(0) - 65]++;
  return c;
}

function stepWords(word, dict) {
  const base = counts(word);
  const res = [];
  for (const w of dict) {
    if (w.length !== word.length + 1) continue;
    const cnt = counts(w);
    let ok = true, diff = 0;
    for (let i = 0; i < 26; i++) {
      if (cnt[i] < base[i]) { ok = false; break; }
      diff += cnt[i] - base[i];
    }
    if (ok && diff === 1 && !w.startsWith(word)) res.push(w);
  }
  return res;
}

const word = "APPLE";
const dict = ["APPEAL", "APPLE", "PEAR", "PALE", "APPEALS", "PAPER", "APPLES", "LAPEL"];
for (const w of stepWords(word, dict)) console.log(w);
