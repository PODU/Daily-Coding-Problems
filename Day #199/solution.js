// Day 199: Balance parentheses with minimum insertions/deletions.
// Greedy: each unmatched paren costs exactly 1 op; inserting its partner is always optimal.
// Time: O(n), Space: O(n).
function balance(s) {
  let res = "";
  let open = 0;
  for (const c of s) {
    if (c === '(') { res += '('; open++; }
    else {
      if (open > 0) { res += ')'; open--; }
      else res += "()"; // unmatched ')': insert a '(' before it
    }
  }
  res += ')'.repeat(open); // close remaining opens
  return res;
}

console.log(balance("(()"));   // (())
console.log(balance("))()(")); // ()()()()
