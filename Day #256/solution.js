// Day 256: Rearrange linked list values into zigzag low->high->low form.
// One pass over node values: at even i ensure a[i]<=a[i+1], at odd i ensure a[i]>=a[i+1], swap on violation.
// Time: O(n), Space: O(1).
"use strict";

class Node {
    constructor(val) {
        this.val = val;
        this.next = null;
    }
}

function wiggle(head) {
    let less = true; // even index: want current <= next
    for (let cur = head; cur && cur.next; cur = cur.next) {
        if (less) {
            if (cur.val > cur.next.val) [cur.val, cur.next.val] = [cur.next.val, cur.val];
        } else {
            if (cur.val < cur.next.val) [cur.val, cur.next.val] = [cur.next.val, cur.val];
        }
        less = !less;
    }
}

function listToString(head) {
    const parts = [];
    for (let cur = head; cur; cur = cur.next) parts.push(cur.val);
    return parts.join(" -> ");
}

function main() {
    const vals = [1, 2, 3, 4, 5];
    let head = null, tail = null;
    for (const v of vals) {
        const n = new Node(v);
        if (!head) { head = tail = n; }
        else { tail.next = n; tail = n; }
    }
    wiggle(head);
    console.log(listToString(head)); // 1 -> 3 -> 2 -> 5 -> 4
}

main();
