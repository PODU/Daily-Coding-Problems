// Day 284: Find all cousins of a node (same level, different parent) via BFS.
// Time O(N), Space O(N).
class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function cousins(root, target) {
  let q = [root];
  while (q.length) {
    const next = [];
    const level = [];
    let targetParent = null;
    for (const n of q) {
      for (const c of [n.left, n.right]) {
        if (c) {
          level.push(c.val);
          if (c.val === target) targetParent = n;
          next.push(c);
        }
      }
    }
    if (targetParent) {
      const sib = new Set();
      if (targetParent.left) sib.add(targetParent.left.val);
      if (targetParent.right) sib.add(targetParent.right.val);
      return level.filter((v) => !sib.has(v));
    }
    q = next;
  }
  return [];
}

const root = new Node(1, new Node(2, new Node(4), new Node(5)), new Node(3, null, new Node(6)));
console.log(cousins(root, 4)); // [6]
