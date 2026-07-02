// Day 1750: Jump game — can advance at most arr[i] steps from index i.
// Greedy: track farthest reachable index in one pass.
// Time O(n), Space O(1).
'use strict';

function canReach(a) {
  let reach = 0;
  for (let i = 0; i < a.length; i++) {
    if (i > reach) return false;
    reach = Math.max(reach, i + a[i]);
  }
  return true;
}

console.log(canReach([1, 3, 1, 2, 0, 1])); // true
console.log(canReach([1, 2, 1, 0, 0]));    // false
