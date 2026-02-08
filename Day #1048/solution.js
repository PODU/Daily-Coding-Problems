// Cartesian tree (min-heap ordered, in-order = S) via linear stack on right spine.
// Build O(n); rotated-90 print + verification. Time O(n), Space O(n).
'use strict';

class Node {
    constructor(v) {
        this.val = v;
        this.left = null;
        this.right = null;
    }
}

function build(s) {
    const st = [];
    for (const v of s) {
        const cur = new Node(v);
        let last = null;
        while (st.length && st[st.length - 1].val > v) {
            last = st.pop();
        }
        cur.left = last;
        if (st.length) st[st.length - 1].right = cur;
        st.push(cur);
    }
    return st.length ? st[0] : null;
}

function printRotated(n, depth) {
    if (!n) return;
    printRotated(n.right, depth + 1);
    console.log(" ".repeat(depth * 4) + n.val);
    printRotated(n.left, depth + 1);
}

function inorder(n, out) {
    if (!n) return;
    inorder(n.left, out);
    out.push(n.val);
    inorder(n.right, out);
}

function minHeap(n) {
    if (!n) return true;
    if (n.left && n.left.val <= n.val) return false;
    if (n.right && n.right.val <= n.val) return false;
    return minHeap(n.left) && minHeap(n.right);
}

const s = [3, 2, 6, 1, 9];
const root = build(s);
console.log(`Cartesian tree (rotated 90 deg, root=${root.val}):`);
printRotated(root, 0);
const out = [];
inorder(root, out);
console.log("in-order:", out.join(" "));
console.log("min-heap ordered:", minHeap(root) ? "true" : "false");
