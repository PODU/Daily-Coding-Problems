// Day 266: Step words. A dict word is a step word of `word` if its length is
// one greater and its letter multiset is a superset of `word`'s (diff = 1).
// Time O(D * L) over dictionary; space O(alphabet).

function isStepWord(word, cand) {
  if (cand.length !== word.length + 1) return false;
  const cnt = new Array(26).fill(0);
  for (const c of cand.toLowerCase()) cnt[c.charCodeAt(0) - 97]++;
  for (const c of word.toLowerCase()) {
    if (--cnt[c.charCodeAt(0) - 97] < 0) return false;
  }
  return cnt.reduce((a, b) => a + b, 0) === 1;
}

function stepWords(word, dict) {
  return dict.filter((w) => isStepWord(word, w));
}

const word = "APPLE";
const dict = ["APPEAL", "APPLES", "KELP", "PALE", "APPLE"];
console.log("[" + stepWords(word, dict).join(", ") + "]");
