// Day 942: Min parentheses to remove to make the string valid.
// One pass: count unmatched ')' immediately + leftover unmatched '('. O(n) time, O(1) space.
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

console.log(minRemovals("()())()")); // 1
console.log(minRemovals(")("));        // 2
