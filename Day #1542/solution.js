// Balanced brackets check using a stack.
// Time O(n), Space O(n).
function isBalanced(s) {
  const st = [];
  const match = { ")": "(", "]": "[", "}": "{" };
  for (const c of s) {
    if (c === "(" || c === "[" || c === "{") st.push(c);
    else if (c in match) {
      if (st.pop() !== match[c]) return false;
    }
  }
  return st.length === 0;
}

console.log(isBalanced("([])[]({})"));
