// Inorder successor in a BST using parent pointers.
// If node has right subtree -> leftmost of right subtree; else walk up parents
// until coming from a left child. Time O(h), Space O(1).

class Node {
  constructor(val) {
    this.val = val;
    this.left = null;
    this.right = null;
    this.parent = null;
  }
}

function inorderSuccessor(node) {
  if (!node) return null;
  if (node.right) {
    let cur = node.right;
    while (cur.left) cur = cur.left;
    return cur;
  }
  let cur = node;
  let p = node.parent;
  while (p && p.right === cur) {
    cur = p;
    p = p.parent;
  }
  return p;
}

function main() {
  const root = new Node(10);
  const n5 = new Node(5);
  const n30 = new Node(30);
  const n22 = new Node(22);
  const n35 = new Node(35);
  root.left = n5; n5.parent = root;
  root.right = n30; n30.parent = root;
  n30.left = n22; n22.parent = n30;
  n30.right = n35; n35.parent = n30;

  const succ = inorderSuccessor(n22);
  console.log(succ ? succ.val : "null");
}

main();
