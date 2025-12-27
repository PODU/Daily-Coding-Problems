// Day 808: In-order traversal of a binary tree using O(1) extra space (Morris).
// Thread predecessor's right pointer to current, then unthread. Time O(N), Space O(1).

class Node {
  constructor(val) {
    this.val = val;
    this.left = null;
    this.right = null;
  }
}

function morrisInorder(root) {
  const out = [];
  let cur = root;
  while (cur) {
    if (!cur.left) {
      out.push(cur.val);
      cur = cur.right;
    } else {
      let pred = cur.left;
      while (pred.right && pred.right !== cur) pred = pred.right;
      if (pred.right === null) {
        pred.right = cur;
        cur = cur.left;
      } else {
        pred.right = null;
        out.push(cur.val);
        cur = cur.right;
      }
    }
  }
  return out;
}

const root = new Node(4);
root.left = new Node(2); root.right = new Node(6);
root.left.left = new Node(1); root.left.right = new Node(3);
root.right.left = new Node(5); root.right.right = new Node(7);
console.log(morrisInorder(root)); // [ 1, 2, 3, 4, 5, 6, 7 ]
