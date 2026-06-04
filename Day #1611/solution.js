// Orderly Queue: move one of the first k letters to the end repeatedly; find lexicographically smallest.
// k==1 -> smallest rotation; k>=2 -> sorted ascending. Time O(n^2) (k==1) / O(n log n), Space O(n).
'use strict';

function smallest(s, k) {
  if (k >= 2) {
    return s.split('').sort().join('');
  }
  // k == 1: smallest rotation
  let best = s;
  for (let i = 1; i < s.length; i++) {
    const rot = s.slice(i) + s.slice(0, i);
    if (rot < best) best = rot;
  }
  return best;
}

function main() {
  console.log(smallest('daily', 1));
}

main();
