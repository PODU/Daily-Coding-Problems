// BST two-sum: in-order traversal -> sorted array, two-pointer scan.
// O(n) time, O(n) space.
class Node {
    constructor(val) {
        this.val = val;
        this.left = null;
        this.right = null;
    }
}

function inorder(r, out) {
    if (!r) return;
    inorder(r.left, out);
    out.push(r.val);
    inorder(r.right, out);
}

function findPair(root, k) {
    const v = [];
    inorder(root, v);
    let i = 0, j = v.length - 1;
    while (i < j) {
        const s = v[i] + v[j];
        if (s === k) return [v[i], v[j]];
        if (s < k) i++; else j--;
    }
    return null;
}

const root = new Node(10);
root.left = new Node(5);
root.right = new Node(15);
root.right.left = new Node(11);
root.right.right = new Node(15);
const [a, b] = findPair(root, 20);
console.log(`${a} and ${b}`);
