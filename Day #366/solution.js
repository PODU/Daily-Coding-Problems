// Day 366: Rearrange so no two adjacent chars match (else null).
// Greedy: repeatedly place the most frequent char that isn't the one just placed.
// Feasible iff maxFreq <= (n+1)/2. Time O(n log k) (here O(n*k) with linear scan).
'use strict';

function reorganize(s) {
  const cnt = {};
  for (const c of s) cnt[c] = (cnt[c] || 0) + 1;
  const res = [];
  let prev = null;
  for (let i = 0; i < s.length; i++) {
    // pick the highest-count char != prev
    let best = null;
    for (const c in cnt) {
      if (cnt[c] > 0 && c !== prev && (best === null || cnt[c] > cnt[best])) best = c;
    }
    if (best === null) return 'null';
    res.push(best);
    cnt[best]--;
    prev = best;
  }
  return res.join('');
}

console.log(reorganize('yyz'));  // yzy
console.log(reorganize('yyy'));  // null
