// BFS level-order traversal of a binary tree using a queue. Time O(n), Space O(n).
class Node {
  constructor(val) {
    this.val = val;
    this.left = null;
    this.right = null;
  }
}

function main() {
  const root = new Node(1);
  root.left = new Node(2);
  root.right = new Node(3);
  root.right.left = new Node(4);
  root.right.right = new Node(5);

  const out = [];
  const q = [root];
  while (q.length) {
    const n = q.shift();
    out.push(n.val);
    if (n.left) q.push(n.left);
    if (n.right) q.push(n.right);
  }
  console.log(out.join(", "));
}

main();
