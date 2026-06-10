// Greedy: track low/high possible open-paren counts in one pass.
// Time O(n), Space O(1). Balanced iff low reaches 0 at end and high never < 0.

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

function main() {
  const a = "(()*", b = "(*)", c = ")*(";
  const ra = isBalanced(a), rb = isBalanced(b), rc = isBalanced(c);
  console.log(`${a} and ${b} are ${ra && rb ? "balanced" : "not balanced"}. `
    + `${c} is ${rc ? "balanced" : "not balanced"}.`);
}

main();
