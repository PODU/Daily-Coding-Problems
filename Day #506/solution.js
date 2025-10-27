// Zigzag rearrange linked list values in a single pass by swapping adjacent
// node values that violate the expected ordering. Time O(n), Space O(1).
'use strict';

class Node {
    constructor(val) {
        this.val = val;
        this.next = null;
    }
}

function zigzag(head) {
    let less = true; // even index expects list[i] <= list[i+1]
    for (let cur = head; cur && cur.next; cur = cur.next) {
        if (less) {
            if (cur.val > cur.next.val) { const t = cur.val; cur.val = cur.next.val; cur.next.val = t; }
        } else {
            if (cur.val < cur.next.val) { const t = cur.val; cur.val = cur.next.val; cur.next.val = t; }
        }
        less = !less;
    }
}

function main() {
    let head = null, tail = null;
    for (const v of [1, 2, 3, 4, 5]) {
        const n = new Node(v);
        if (!head) head = tail = n;
        else { tail.next = n; tail = n; }
    }
    zigzag(head);
    const parts = [];
    for (let cur = head; cur; cur = cur.next) parts.push(cur.val);
    console.log(parts.join(' -> '));
}

main();
