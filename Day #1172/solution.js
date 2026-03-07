// Day 1172: Reconstruct a binary tree from pre-order and in-order traversals.
// Recursion with a hashmap of in-order positions; first pre-order element is the
// root, its in-order index splits left/right subtrees. Time O(N), Space O(N).

class Node {
    constructor(val) { this.val = val; this.left = null; this.right = null; }
}

function reconstruct(preorder, inorder) {
    const pos = new Map(inorder.map((v, i) => [v, i]));
    let pi = 0;
    function build(lo, hi) {
        if (lo > hi) return null;
        const rootVal = preorder[pi++];
        const root = new Node(rootVal);
        const m = pos.get(rootVal);
        root.left = build(lo, m - 1);
        root.right = build(m + 1, hi);
        return root;
    }
    return build(0, inorder.length - 1);
}

function inorderStr(node) {
    return node ? inorderStr(node.left) + node.val + inorderStr(node.right) : "";
}

const pre = ['a', 'b', 'd', 'e', 'c', 'f', 'g'];
const ino = ['d', 'b', 'e', 'a', 'f', 'c', 'g'];
const root = reconstruct(pre, ino);
console.assert(inorderStr(root) === ino.join(""));   // verifies reconstruction
console.log("    a");
console.log("   / \\");
console.log("  b   c");
console.log(" / \\ / \\");
console.log("d  e f  g");
