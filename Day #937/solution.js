// Day 937: Valid parenthesis string with '*' (= '(' , ')' or empty).
// Greedy: track [lo,hi] range of possible open counts. Valid iff lo can reach 0. O(n) time, O(1) space.
function isValid(s) {
  let lo = 0, hi = 0;
  for (const c of s) {
    if (c === '(') { lo++; hi++; }
    else if (c === ')') { lo--; hi--; }
    else { lo--; hi++; }
    if (hi < 0) return false;
    if (lo < 0) lo = 0;
  }
  return lo === 0;
}

const inputs = ["(()*", "(*)", ")*("];
const bal = inputs.filter(isValid);
const notbal = inputs.filter((s) => !isValid(s));
console.log(`${bal.join(" and ")} are balanced. ${notbal.join(" and ")} is not balanced.`);
// (()* and (*) are balanced. )*( is not balanced.
