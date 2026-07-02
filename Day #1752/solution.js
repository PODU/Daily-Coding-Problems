// Day 1752: Remove kth-from-last node of a singly linked list in ONE pass, O(1) space.
// Two pointers spaced k apart; when fast reaches end, slow is just before the target. O(n) time.
// Assumption (no README example): list 1->2->3->4->5, k=2 removes value 4 -> "1 2 3 5".
'use strict';

class Node {
  constructor(val) {
    this.val = val;
    this.next = null;
  }
}

function removeKthLast(head, k) {
  const dummy = new Node(0);
  dummy.next = head;
  let fast = dummy, slow = dummy;
  for (let i = 0; i < k; i++) fast = fast.next; // advance fast k steps
  while (fast.next) { fast = fast.next; slow = slow.next; }
  slow.next = slow.next.next; // unlink target
  return dummy.next;
}

function build(values) {
  const dummy = new Node(0);
  let cur = dummy;
  for (const v of values) { cur.next = new Node(v); cur = cur.next; }
  return dummy.next;
}

let head = build([1, 2, 3, 4, 5]);
head = removeKthLast(head, 2);
const out = [];
for (let p = head; p; p = p.next) out.push(p.val);
console.log(out.join(' ')); // 1 2 3 5
