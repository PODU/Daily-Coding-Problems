// Count unival subtrees: postorder; a subtree is unival if both children are
// unival and match the node's value. Time: O(n), Space: O(h).
class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function countUnival(root) {
  let count = 0;
  function isUnival(n) {
    if (n === null) return true;
    const l = isUnival(n.left);
    const r = isUnival(n.right);
    if (!l || !r) return false;
    if (n.left && n.left.val !== n.val) return false;
    if (n.right && n.right.val !== n.val) return false;
    count++;
    return true;
  }
  isUnival(root);
  return count;
}

const root = new Node(0, new Node(1), new Node(0, new Node(1, new Node(1), new Node(1)), new Node(0)));
console.log(countUnival(root));
