// Cryptarithmetic solver via backtracking over distinct letters with column-sum check.
// Time: O(10!) worst case over distinct letters (<=10), Space: O(#letters).
'use strict';

function solveCryptarithm(word1, word2, word3) {
  const seen = [];
  for (const c of word1 + word2 + word3) if (!seen.includes(c)) seen.push(c);
  const leading = new Set([word1[0], word2[0], word3[0]]);
  const assign = {};
  const used = new Array(10).fill(false);

  const value = (w) => {
    let v = 0;
    for (const c of w) v = v * 10 + assign[c];
    return v;
  };

  function backtrack(idx) {
    if (idx === seen.length)
      return value(word1) + value(word2) === value(word3);
    const ch = seen[idx];
    for (let d = 0; d <= 9; d++) {
      if (used[d]) continue;
      if (d === 0 && leading.has(ch)) continue;
      used[d] = true; assign[ch] = d;
      if (backtrack(idx + 1)) return true;
      used[d] = false;
    }
    return false;
  }

  if (!backtrack(0)) return null;
  const res = {};
  for (const c of seen) res[c] = assign[c];
  return res;
}

if (require.main === module) {
  const res = solveCryptarithm("SEND", "MORE", "MONEY");
  const body = Object.keys(res).map(c => `'${c}': ${res[c]}`).join(', ');
  console.log(`{${body}}`);
}
