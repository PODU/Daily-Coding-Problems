// Reverse singly linked list in place: iterate with prev/curr/next pointers. Time O(n), Space O(1).
'use strict';

class ListNode {
    constructor(val) {
        this.val = val;
        this.next = null;
    }
}

function reverseList(head) {
    let prev = null;
    while (head) {
        const nxt = head.next;
        head.next = prev;
        prev = head;
        head = nxt;
    }
    return prev;
}

function main() {
    let head = new ListNode(1);
    head.next = new ListNode(2);
    head.next.next = new ListNode(3);
    head.next.next.next = new ListNode(4);
    head.next.next.next.next = new ListNode(5);

    head = reverseList(head);

    const vals = [];
    for (let p = head; p; p = p.next) vals.push(p.val);
    console.log(vals.join(' '));
}

main();
