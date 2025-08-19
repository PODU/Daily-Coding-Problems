// Balanced parens with '*' wildcard: track range [lo,hi] of possible open counts. O(n) time, O(1) space.

function isBalanced(s) {
  let lo = 0, hi = 0;
  for (const c of s) {
    if (c === '(') { lo++; hi++; }
    else if (c === ')') { lo--; hi--; }
    else { lo--; hi++; } // '*' as ')', '(' or empty
    if (hi < 0) return false;
    if (lo < 0) lo = 0;
  }
  return lo === 0;
}

const a = "(()*", b = "(*)", c = ")*(";
console.log(
  `${isBalanced(a) ? "(()*" : ""} and ${isBalanced(b) ? "(*)" : ""} are balanced. ` +
  `${!isBalanced(c) ? ")*(" : ""} is not balanced.`
);
