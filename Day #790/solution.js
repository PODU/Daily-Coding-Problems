// Step words: a dict word qualifies if len==word.len+1 and word's letter
// multiset is a subset leaving exactly one extra letter. O(dict*wordLen) time, O(1) space.
function stepWords(word, dict) {
  const base = new Array(26).fill(0);
  for (const c of word) base[c.charCodeAt(0) - 65]++;
  const res = [];
  for (const w of dict) {
    if (w.length !== word.length + 1) continue;
    const cnt = new Array(26).fill(0);
    for (const c of w) cnt[c.charCodeAt(0) - 65]++;
    let extra = 0, ok = true;
    for (let i = 0; i < 26; i++) {
      const d = cnt[i] - base[i];
      if (d < 0) { ok = false; break; }
      extra += d;
    }
    if (ok && extra === 1) res.push(w);
  }
  return res;
}

const word = "APPLE";
const dict = ["APPEAL", "BANANA", "ORANGE", "GRAPE"];
console.log(stepWords(word, dict).join(" "));
