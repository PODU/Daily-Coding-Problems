// Day 751: In-order traversal with O(1) extra space via Morris Traversal.
// Time: O(n), Space: O(1) (re-uses null right pointers as temporary threads).
"use strict";

class Node {
    constructor(val, left = null, right = null) {
        this.val = val;
        this.left = left;
        this.right = right;
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
            let pre = cur.left;
            while (pre.right && pre.right !== cur) pre = pre.right;
            if (!pre.right) {            // create thread
                pre.right = cur;
                cur = cur.left;
            } else {                     // remove thread and visit
                pre.right = null;
                out.push(cur.val);
                cur = cur.right;
            }
        }
    }
    return out;
}

const root = new Node(4,
    new Node(2, new Node(1), new Node(3)),
    new Node(6, new Node(5), new Node(7)));

console.log(morrisInorder(root).join(" "));
