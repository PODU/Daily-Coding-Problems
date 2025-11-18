// Deepest node in a binary tree via BFS level order; last visited node is deepest.
// Time O(N), Space O(N).
class Node {
  constructor(val) {
    this.val = val;
    this.left = null;
    this.right = null;
  }
}

function deepest(root) {
  if (!root) return null;
  const q = [root];
  let last = root;
  while (q.length) {
    last = q.shift();
    if (last.left) q.push(last.left);
    if (last.right) q.push(last.right);
  }
  return last.val;
}

const a = new Node('a');
const b = new Node('b');
const c = new Node('c');
const d = new Node('d');
a.left = b; a.right = c;
b.left = d;
console.log(deepest(a)); // d
