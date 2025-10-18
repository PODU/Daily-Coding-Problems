// Day 450: Balanced parentheses with '*' wildcards via greedy low/high open
// count. O(n) time, O(1) space.

function isBalanced(s) {
  let low = 0, high = 0;
  for (const c of s) {
    if (c === '(') { low++; high++; }
    else if (c === ')') { low--; high--; }
    else { low--; high++; }
    if (high < 0) return false;
    if (low < 0) low = 0;
  }
  return low === 0;
}

const s = "(()*";
console.log(isBalanced(s) ? "balanced" : "not balanced"); // balanced
