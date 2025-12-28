// Day 809: Check balanced round/curly/square brackets using a stack.
// Push opens, match closes against stack top. Time O(N), Space O(N).

function isBalanced(s) {
  const stack = [];
  const match = { ")": "(", "]": "[", "}": "{" };
  for (const c of s) {
    if (c === "(" || c === "[" || c === "{") stack.push(c);
    else if (c in match) {
      if (stack.pop() !== match[c]) return false;
    }
  }
  return stack.length === 0;
}

console.log(isBalanced("([])[]({})")); // true
console.log(isBalanced("([)]"));        // false
console.log(isBalanced("((()"));        // false
