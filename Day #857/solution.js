// Day 857: Depth of tree from (lr) string representation.
// Approach: depth equals the maximum nesting level of parentheses.
// Time: O(n), Space: O(1).
'use strict';

function depth(s) {
  let cur = 0, mx = 0;
  for (const c of s) {
    if (c === '(') { cur++; if (cur > mx) mx = cur; }
    else if (c === ')') cur--;
  }
  return mx;
}

console.log(depth('(00)'));            // 1
console.log(depth('((00)(00))'));      // 2
console.log(depth('((((00)0)0)0)'));   // 4
