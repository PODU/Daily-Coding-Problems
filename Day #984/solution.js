// Balance a parentheses string with minimum insertions+deletions (insertion-only
// greedy is provably optimal: each unmatched paren needs exactly one edit).
// Time: O(n), Space: O(n).

function balance(s) {
  let res = "";
  let bal = 0;
  for (const c of s) {
    if (c === "(") {
      res += "(";
      bal++;
    } else { // ')'
      if (bal > 0) {
        res += ")";
        bal--;
      } else {
        res += "()"; // insert '(' to match this ')'
      }
    }
  }
  res += ")".repeat(bal); // close any still-open '('
  return res;
}

console.log(balance("(()"));
console.log(balance("))()("));
