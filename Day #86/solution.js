// Day 86: Min parentheses to remove for validity. Track unmatched '(' and ')'.
// Time O(n), Space O(1).
function minRemoval(s) {
  let open = 0, removals = 0;
  for (const c of s) {
    if (c === "(") open++;
    else if (c === ")") {
      if (open > 0) open--;
      else removals++; // unmatched ')'
    }
  }
  return removals + open; // leftover unmatched '('
}

console.log(minRemoval("()())()")); // 1
console.log(minRemoval(")(")); // 2
