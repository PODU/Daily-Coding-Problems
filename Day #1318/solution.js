// Move one of first k chars to the end, unlimited times.
// k==1: only rotations reachable -> smallest rotation. k>=2: any order -> sort.
// Time O(n^2) for k==1 (rotation scan), O(n log n) for k>=2.
'use strict';

function smallestString(s, k) {
  if (k >= 2) return s.split('').sort().join('');
  let best = s;
  for (let i = 1; i < s.length; i++) {
    const rot = s.slice(i) + s.slice(0, i);
    if (rot < best) best = rot;
  }
  return best;
}

console.log(smallestString("daily", 1)); // ailyd
