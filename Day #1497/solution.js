// Day 1497: Step words. Dict word w is a step word of s if len(w)==len(s)+1
// and multiset(s) subset of multiset(w). Char-count comparison.
// Time O(D*L), Space O(1) (26-letter alphabet).
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
      const diff = cnt[i] - base[i];
      if (diff < 0) { ok = false; break; }
      extra += diff;
    }
    if (ok && extra === 1) res.push(w);
  }
  return res;
}

const input = "APPLE";
const dict = ["APPEAL", "APPLE", "BANANA", "PLEASE", "APPEALS"];
console.log(JSON.stringify(stepWords(input, dict)));
