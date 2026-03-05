// Bottom-up iterative merge sort on a singly linked list. O(n log n) time, O(1) auxiliary space.
"use strict";

class ListNode {
    constructor(val) {
        this.val = val;
        this.next = null;
    }
}

// Take `n` nodes from head, cut, return remainder.
function split(head, n) {
    for (let i = 1; head && i < n; i++) head = head.next;
    if (head === null) return null;
    const second = head.next;
    head.next = null;
    return second;
}

// Merge sorted lists a, b onto tail; return new tail.
function merge(a, b, tail) {
    let cur = tail;
    while (a && b) {
        if (a.val <= b.val) { cur.next = a; a = a.next; }
        else { cur.next = b; b = b.next; }
        cur = cur.next;
    }
    cur.next = a ? a : b;
    while (cur.next) cur = cur.next;
    return cur;
}

function sortList(head) {
    if (head === null || head.next === null) return head;
    let n = 0;
    for (let p = head; p; p = p.next) n++;

    const dummy = new ListNode(0);
    dummy.next = head;
    for (let size = 1; size < n; size <<= 1) {
        let cur = dummy.next;
        let tail = dummy;
        while (cur) {
            const left = cur;
            const right = split(left, size);
            cur = split(right, size);
            tail = merge(left, right, tail);
        }
    }
    return dummy.next;
}

function main() {
    const vals = [4, 1, -3, 99];
    let head = null, tail = null;
    for (const v of vals) {
        const node = new ListNode(v);
        if (head === null) { head = tail = node; }
        else { tail.next = node; tail = node; }
    }

    head = sortList(head);

    const parts = [];
    for (let p = head; p; p = p.next) parts.push(String(p.val));
    console.log(parts.join(" -> "));
}

main();
