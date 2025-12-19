// Day 765: Remove the kth-from-last node in one pass with two pointers.
// fast leads slow by k; when fast hits the end, slow precedes the target.
// Time: O(n), Space: O(1).
"use strict";

class Node {
    constructor(val, next = null) { this.val = val; this.next = next; }
}

function removeKthLast(head, k) {
    const dummy = new Node(0, head);
    let fast = dummy, slow = dummy;
    for (let i = 0; i < k; i++) fast = fast.next;
    while (fast.next) { fast = fast.next; slow = slow.next; }
    slow.next = slow.next.next;   // unlink target
    return dummy.next;
}

function printList(head) {
    const parts = [];
    for (let p = head; p; p = p.next) parts.push(p.val);
    console.log(parts.join(" -> "));
}

let head = new Node(1);
let cur = head;
for (let v = 2; v <= 5; v++) { cur.next = new Node(v); cur = cur.next; }

process.stdout.write("before: "); printList(head);
head = removeKthLast(head, 2);
process.stdout.write("after:  "); printList(head);   // 1 -> 2 -> 3 -> 5
