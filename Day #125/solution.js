// Day 125: Two nodes in a BST summing to K.
// Inorder traversal -> sorted values, two-pointer. O(n) time, O(n) space.
class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function inorder(r, out) {
  if (!r) return;
  inorder(r.left, out);
  out.push(r.val);
  inorder(r.right, out);
}

function twoSum(root, k) {
  const v = [];
  inorder(root, v);
  let i = 0, j = v.length - 1;
  while (i < j) {
    const s = v[i] + v[j];
    if (s === k) return [v[i], v[j]];
    if (s < k) i++;
    else j--;
  }
  return null;
}

const root = new Node(10, new Node(5), new Node(15, new Node(11), new Node(15)));
const [a, b] = twoSum(root, 20);
console.log(`Return the nodes ${a} and ${b}.`);
