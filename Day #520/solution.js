// Tree depth = max nesting level of '(' in the representation. O(n) time, O(1) space.
function treeDepth(s) {
  let depth = 0, best = 0;
  for (const c of s) {
    if (c === '(') best = Math.max(best, ++depth);
    else if (c === ')') depth--;
  }
  return best;
}

console.log(treeDepth("((((00)0)0)0)")); // 4
