// BFS level order; last node dequeued is a deepest node. Time O(n), Space O(n).

class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function deepestNode(root) {
  if (!root) return null;
  const q = [root];
  let last = root;
  let head = 0;
  while (head < q.length) {
    last = q[head++];
    if (last.left) q.push(last.left);
    if (last.right) q.push(last.right);
  }
  return last;
}

const a = new Node('a');
const b = new Node('b');
const c = new Node('c');
const d = new Node('d');
a.left = b; a.right = c;
b.left = d;
console.log(deepestNode(a).val);
