// Balanced brackets via stack. Time O(n), Space O(n).
function isBalanced(s) {
  const stack = [];
  const match = { ')': '(', ']': '[', '}': '{' };
  for (const c of s) {
    if (c === '(' || c === '[' || c === '{') {
      stack.push(c);
    } else if (c in match) {
      if (stack.pop() !== match[c]) return false;
    }
  }
  return stack.length === 0;
}

console.assert(isBalanced("([)]") === false);
console.assert(isBalanced("((()") === false);
console.log(isBalanced("([])[]({})") ? "true" : "false");
