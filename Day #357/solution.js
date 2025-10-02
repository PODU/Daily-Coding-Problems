// Tree depth from nested-paren string: scan, track paren nesting, return max depth. O(N) time, O(1) space.
function treeDepth(s) {
  let depth = 0, maxDepth = 0;
  for (const c of s) {
    if (c === '(') { depth++; if (depth > maxDepth) maxDepth = depth; }
    else if (c === ')') depth--;
  }
  return maxDepth;
}
console.log(treeDepth("((((00)0)0)0)"));
