// Day 1681: Min parentheses to remove for validity. Track unmatched '(' and count
// unmatched ')'; answer = leftover open + unmatched close. Time O(n), Space O(1).
function minRemovals(s) {
  let open = 0, removals = 0;
  for (const c of s) {
    if (c === '(') open++;
    else if (c === ')') { if (open > 0) open--; else removals++; }
  }
  return removals + open;
}

console.log(minRemovals("()())()")); // 1
console.log(minRemovals(")("));       // 2
