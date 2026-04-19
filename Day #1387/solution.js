// BST two-sum: in-order into sorted array, then two-pointer scan for pair summing to K. O(n) time, O(n) space.
class Node {
    constructor(val) { this.val = val; this.left = null; this.right = null; }
}

function inorder(root, a) {
    if (!root) return;
    inorder(root.left, a);
    a.push(root.val);
    inorder(root.right, a);
}

function twoSum(root, k) {
    const a = [];
    inorder(root, a);
    let i = 0, j = a.length - 1;
    while (i < j) {
        const s = a[i] + a[j];
        if (s === k) return [a[i], a[j]];
        if (s < k) i++; else j--;
    }
    return [-1, -1];
}

const root = new Node(10);
root.left = new Node(5);
root.right = new Node(15);
root.right.left = new Node(11);
root.right.right = new Node(15);
const [x, y] = twoSum(root, 20);
console.log(x + " " + y);
