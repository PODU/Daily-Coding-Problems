// Balance parentheses with min insertions/deletions via insertion-based scan.
// Time O(n), Space O(n).
function balance(s) {
  const result = [];
  let open = 0;
  for (const c of s) {
    if (c === '(') { result.push('('); open++; }
    else { // ')'
      if (open === 0) { result.push('('); result.push(')'); }
      else { result.push(')'); open--; }
    }
  }
  result.push(')'.repeat(open));
  return result.join('');
}

console.log(balance("(()"));
console.log(balance("))()("));
