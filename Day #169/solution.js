// Bottom-up (iterative) merge sort on a singly linked list. O(n log n) time, O(1) space.

class Node {
    constructor(val, next = null) { this.val = val; this.next = next; }
}

function merge(a, b) {
    const dummy = new Node(0);
    let tail = dummy;
    while (a && b) {
        if (a.val <= b.val) { tail.next = a; a = a.next; }
        else { tail.next = b; b = b.next; }
        tail = tail.next;
    }
    tail.next = a ? a : b;
    while (tail.next) tail = tail.next;
    return [dummy.next, tail];
}

function length(h) { let n = 0; while (h) { n++; h = h.next; } return n; }

function split(head, n) {
    for (let i = 1; head && i < n; i++) head = head.next;
    if (!head) return null;
    const rest = head.next;
    head.next = null;
    return rest;
}

function sortList(head) {
    const n = length(head);
    const dummy = new Node(0);
    dummy.next = head;
    for (let size = 1; size < n; size <<= 1) {
        let cur = dummy.next;
        let tail = dummy;
        while (cur) {
            const left = cur;
            const right = split(left, size);
            cur = split(right, size);
            const [mHead, mTail] = merge(left, right);
            tail.next = mHead;
            tail = mTail;
        }
    }
    return dummy.next;
}

function main() {
    const vals = [4, 1, -3, 99];
    const dummy = new Node(0);
    let t = dummy;
    for (const v of vals) { t.next = new Node(v); t = t.next; }
    const head = sortList(dummy.next);
    const parts = [];
    for (let p = head; p; p = p.next) parts.push(p.val);
    console.log(parts.join(" -> "));
}

main();
