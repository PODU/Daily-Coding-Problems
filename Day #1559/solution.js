// Scan parenthesis string, track open-paren depth, record maximum. Time O(n), Space O(1).
function treeDepth(s) {
  let depth = 0, maxDepth = 0;
  for (const c of s) {
    if (c === '(') { depth++; maxDepth = Math.max(maxDepth, depth); }
    else if (c === ')') depth--;
  }
  return maxDepth;
}

console.log(treeDepth("((((00)0)0)0)"));
