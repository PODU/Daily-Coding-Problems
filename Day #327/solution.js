// Merge two binary trees recursively (sum overlaps, keep lone nodes); print merged tree level-order skipping nulls.
// Time: O(n), Space: O(n).
class Node {
  constructor(val) {
    this.val = val;
    this.left = null;
    this.right = null;
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

function main() {
  const t1 = new Node(1);
  t1.left = new Node(3); t1.right = new Node(2);
  t1.left.left = new Node(5);

  const t2 = new Node(2);
  t2.left = new Node(1); t2.right = new Node(3);
  t2.left.right = new Node(4);
  t2.right.right = new Node(7);

  const m = merge(t1, t2);

  let level = [m];
  while (level.length) {
    const line = level.map((c) => c.val).join(" ");
    console.log(line);
    const next = [];
    for (const c of level) {
      if (c.left) next.push(c.left);
      if (c.right) next.push(c.right);
    }
    level = next;
  }
}

main();
