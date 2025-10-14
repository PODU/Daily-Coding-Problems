// Day 430: Balance parentheses with the minimum number of insertions + deletions.
// One pass: keep matched pairs; unmatched ')' -> "()", leftover '(' gets a ')'. Time O(n), Space O(n).
function balance(s) {
  let res = [], open = 0;
  for (const c of s) {
    if (c === '(') {
      open++;
      res.push('(');
    } else { // ')'
      if (open > 0) {
        open--;
        res.push(')');
      } else {
        res.push('(', ')');
      }
    }
  }
  while (open-- > 0) res.push(')');
  return res.join('');
}

console.log(balance("(()"));
console.log(balance("))()("));
