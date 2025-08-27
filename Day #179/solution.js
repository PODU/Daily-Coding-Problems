// Reconstruct BST from postorder: iterate right-to-left as (root,right,left) with a lower bound. O(n) time/space.
class Node { constructor(v){ this.val = v; this.left = null; this.right = null; } }

let post = [2, 4, 3, 8, 7, 5];
let idx = post.length - 1;

function build(lower) {
    if (idx < 0 || post[idx] < lower) return null;
    const val = post[idx]; idx--;
    const node = new Node(val);
    node.right = build(val);
    node.left  = build(lower);
    return node;
}

function inorder(n, o) { if (n) { inorder(n.left, o); o.push(n.val); inorder(n.right, o); } }
function postorder(n, o) { if (n) { postorder(n.left, o); postorder(n.right, o); o.push(n.val); } }

const root = build(-Infinity);
const ino = [], po = [];
inorder(root, ino);
postorder(root, po);
console.log("Inorder: " + ino.join(" "));
console.log("Postorder: " + po.join(" "));
