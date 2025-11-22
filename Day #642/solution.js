// Day 642: Step words (add one letter + anagram).
// Approach: a dict word qualifies if len == len(word)+1 and its letter counts
// minus the input's are all >= 0 with exactly one extra letter.
// Time: O(D * L), Space: O(1) (26-letter counts).
function isStep(word, cand) {
  if (cand.length !== word.length + 1) return false;
  const cnt = new Array(26).fill(0);
  for (const c of cand) cnt[c.charCodeAt(0) - 65]++;
  for (const c of word) if (--cnt[c.charCodeAt(0) - 65] < 0) return false;
  return cnt.reduce((a, b) => a + b, 0) === 1;
}

function stepWords(word, dict) {
  return dict.filter((w) => isStep(word, w));
}

const word = "APPLE";
const dict = ["APPEAL", "APPEAR", "PEAR", "APPLES", "PALE"];
console.log(stepWords(word, dict)); // ['APPEAL', 'APPLES']
