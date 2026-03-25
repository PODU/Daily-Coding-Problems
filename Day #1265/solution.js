// Day 1265: Reconstruct a sentence from a space-free string and a dictionary.
// DP over prefixes storing one valid breakpoint. O(n^2) time (avg), O(n) space.
function wordBreak(s, words) {
  const dict = new Set(words);
  const n = s.length;
  const back = new Array(n + 1).fill(-2);
  back[0] = -1;
  for (let i = 1; i <= n; i++) {
    for (let j = 0; j < i; j++) {
      if (back[j] !== -2 && dict.has(s.slice(j, i))) { back[i] = j; break; }
    }
  }
  if (back[n] === -2) return null;
  const res = [];
  for (let i = n; i > 0; i = back[i]) res.unshift(s.slice(back[i], i));
  return res;
}

console.log(wordBreak("thequickbrownfox", ["quick", "brown", "the", "fox"]));
