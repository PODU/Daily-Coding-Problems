// Day 1855: LCA in a binary tree with parent pointers.
// Two-pointer walk (like linked-list intersection): swap to the other node at root; meet at LCA. O(h) time, O(1) space.

class Node {
  constructor(val) {
    this.val = val;
    this.left = null;
    this.right = null;
    this.parent = null;
  }
}

function lca(p, q) {
  let a = p, b = q;
  while (a !== b) {
    a = a === null ? q : a.parent;
    b = b === null ? p : b.parent;
  }
  return a;
}

function attach(parent, child) {
  if (child) child.parent = parent;
  return child;
}

const n = {};
for (let i = 1; i <= 8; i++) n[i] = new Node(i);
n[1].left = attach(n[1], n[2]); n[1].right = attach(n[1], n[3]);
n[2].left = attach(n[2], n[4]); n[2].right = attach(n[2], n[5]);
n[3].right = attach(n[3], n[6]);
n[5].left = attach(n[5], n[7]); n[5].right = attach(n[5], n[8]);

console.log(lca(n[7], n[8]).val); // 5
console.log(lca(n[7], n[6]).val); // 1
console.log(lca(n[4], n[8]).val); // 2
