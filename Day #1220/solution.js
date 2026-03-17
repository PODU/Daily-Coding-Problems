// Merge two binary trees recursively (sum overlaps, keep lone nodes). O(min(n1,n2)) time.
// Serialize merged tree as BFS level-order with trailing nulls trimmed.
class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function merge(a, b) {
  if (a === null) return b;
  if (b === null) return a;
  const n = new Node(a.val + b.val);
  n.left = merge(a.left, b.left);
  n.right = merge(a.right, b.right);
  return n;
}

function serialize(root) {
  const out = [];
  const q = [root];
  let i = 0;
  while (i < q.length) {
    const n = q[i++];
    if (n === null) {
      out.push("null");
      continue;
    }
    out.push(String(n.val));
    q.push(n.left);
    q.push(n.right);
  }
  while (out.length && out[out.length - 1] === "null") out.pop();
  return "[" + out.join(", ") + "]";
}

const t1 = new Node(1, new Node(3, new Node(5)), new Node(2));
const t2 = new Node(2, new Node(1, null, new Node(4)), new Node(3, null, new Node(7)));
console.log(serialize(merge(t1, t2)));
