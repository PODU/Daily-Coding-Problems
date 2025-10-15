// Day 434: BST floor (largest <= n) and ceiling (smallest >= n).
// Single O(h) walk: node.val < n -> floor candidate (go right); node.val > n -> ceiling
// candidate (go left); equal -> both are n. O(h) time, O(1) space.
class Node {
    constructor(v) { this.val = v; this.left = null; this.right = null; }
}

function insert(root, v) {
    if (root === null) return new Node(v);
    if (v < root.val) root.left = insert(root.left, v);
    else root.right = insert(root.right, v);
    return root;
}

function floorCeil(root, n) {
    let f = null, c = null, cur = root;
    while (cur) {
        if (cur.val === n) return [n, n];
        else if (cur.val < n) { f = cur.val; cur = cur.right; }
        else { c = cur.val; cur = cur.left; }
    }
    return [f, c];
}

let root = null;
for (const v of [8, 4, 12, 2, 6, 10, 14]) root = insert(root, v);
for (const n of [5, 8, 15, 1]) {
    const [f, c] = floorCeil(root, n);
    console.log(`n=${n}: floor=${f === null ? "None" : f}, ceiling=${c === null ? "None" : c}`);
}
