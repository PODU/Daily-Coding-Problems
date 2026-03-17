// Single linear scan; depth = max paren nesting ('(' +1, ')' -1).
// O(n) time, O(1) space.
function treeDepth(s) {
  let depth = 0, cur = 0;
  for (const c of s) {
    if (c === "(") { cur++; depth = Math.max(depth, cur); }
    else if (c === ")") cur--;
  }
  return depth;
}

console.log(treeDepth("((((00)0)0)0)"));
