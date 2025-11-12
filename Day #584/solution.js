// Greedy rearrange: at each step pick highest remaining count != prev char, tie by smallest char.
// Feasible iff maxCount <= (n+1)/2. Time O(n*26), Space O(26).
"use strict";

function rearrange(s) {
  const cnt = new Array(26).fill(0);
  for (const c of s) cnt[c.charCodeAt(0) - 97]++;
  const n = s.length;
  let res = "";
  let prev = -1;
  for (let k = 0; k < n; k++) {
    let best = -1;
    for (let i = 0; i < 26; i++) {
      if (i === prev || cnt[i] <= 0) continue;
      if (best === -1 || cnt[i] > cnt[best]) best = i;
    }
    if (best === -1) return null;
    res += String.fromCharCode(97 + best);
    cnt[best]--;
    prev = best;
  }
  return res;
}

const a = rearrange("aaabbc");
console.log(a === null ? "None" : a);
const b = rearrange("aaab");
console.log(b === null ? "None" : b);
