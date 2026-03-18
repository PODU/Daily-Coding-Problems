// Min parens to remove for validity: single pass counting unmatched.
// Time O(n), Space O(1).
function minRemovals(s) {
  let open = 0, removals = 0;
  for (const c of s) {
    if (c === '(') open++;
    else if (c === ')') {
      if (open > 0) open--;
      else removals++;
    }
  }
  return removals + open;
}

console.log(minRemovals("()())()"));
console.log(minRemovals(")("));
