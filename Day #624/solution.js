// Minimum parentheses to remove to make string valid: single pass counting unmatched
// open/close. Answer = removals (unmatched ')') + leftover open. Time O(n), Space O(1).
function minRemove(s) {
  let open = 0;
  let removals = 0;
  for (const c of s) {
    if (c === '(') open++;
    else if (c === ')') {
      if (open > 0) open--;
      else removals++;
    }
  }
  return removals + open;
}

console.log(minRemove("()())()")); // 1
console.log(minRemove(")("));        // 2
