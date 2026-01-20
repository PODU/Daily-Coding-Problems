// Rotate list right by k: find length L, make a ring, break at L-(k%L).
// Time O(n), Space O(1).
'use strict';

class Node {
    constructor(val, next = null) {
        this.val = val;
        this.next = next;
    }
}

function build(vals) {
    let head = null;
    for (let i = vals.length - 1; i >= 0; i--) head = new Node(vals[i], head);
    return head;
}

function toStr(head) {
    const parts = [];
    while (head) { parts.push(head.val); head = head.next; }
    return parts.join(' -> ');
}

function rotateRight(head, k) {
    if (!head || !head.next) return head;
    let L = 1, tail = head;
    while (tail.next) { tail = tail.next; L++; }
    k %= L;
    if (k === 0) return head;
    tail.next = head; // ring
    const steps = L - k;
    let newTail = head;
    for (let i = 0; i < steps - 1; i++) newTail = newTail.next;
    const newHead = newTail.next;
    newTail.next = null;
    return newHead;
}

const head = build([1, 2, 3, 4, 5]);
console.log(toStr(rotateRight(head, 3)));
