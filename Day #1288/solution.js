// Day 1288: Balance parentheses with minimum insertions+deletions (insertion-only is minimal).
// Single scan: pair each ')' with an open, else insert '(' before it; close leftover opens. O(n).
function balance(s) {
  let res = "";
  let open = 0;
  for (const ch of s) {
    if (ch === "(") { res += "("; open++; }
    else {
      if (open > 0) { res += ")"; open--; }
      else { res += "()"; } // insert matching '(' before unmatched ')'
    }
  }
  res += ")".repeat(open);
  return res;
}

console.log(balance("(()"));   // (())
console.log(balance("))()(")); // ()()()()
