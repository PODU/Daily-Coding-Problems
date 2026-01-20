// Sort a linked list in O(n log n) time, O(1) extra space.
// Bottom-up (iterative) merge sort on the list; no recursion stack -> constant space.
'use strict';

class Node {
    constructor(val, next = null) {
        this.val = val;
        this.next = next;
    }
}

function split(head, size) {
    for (let i = 1; head && i < size; i++) head = head.next;
    if (!head) return null;
    const rest = head.next;
    head.next = null;
    return rest;
}

function merge(a, b, tail) {
    while (a && b) {
        if (a.val <= b.val) { tail.next = a; a = a.next; }
        else { tail.next = b; b = b.next; }
        tail = tail.next;
    }
    tail.next = a ? a : b;
    while (tail.next) tail = tail.next;
    return tail;
}

function sortList(head) {
    if (!head || !head.next) return head;
    let n = 0;
    for (let p = head; p; p = p.next) n++;
    const dummy = new Node(0, head);
    for (let size = 1; size < n; size *= 2) {
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

function build(vals) {
    const dummy = new Node(0);
    let t = dummy;
    for (const v of vals) { t.next = new Node(v); t = t.next; }
    return dummy.next;
}

function toStr(head) {
    const parts = [];
    for (let p = head; p; p = p.next) parts.push(p.val);
    return parts.join(' -> ');
}

console.log(toStr(sortList(build([4, 1, -3, 99]))));
