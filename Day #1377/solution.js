// Balanced parens with '*' wildcard: greedy track [lo,hi] of possible open counts.
// Time O(n), Space O(1).
function isValid(s) {
  let lo = 0, hi = 0;
  for (const c of s) {
    if (c === "(") { lo++; hi++; }
    else if (c === ")") { lo--; hi--; }
    else { lo--; hi++; }
    if (hi < 0) return false;
    if (lo < 0) lo = 0;
  }
  return lo === 0;
}

const tests = ["(()*", "(*)", ")*("];
const bal = tests.filter(isValid);
const notBal = tests.filter((s) => !isValid(s));
console.log(bal.join(" and ") + " are balanced. " + notBal.join(" and ") + " is not balanced.");
