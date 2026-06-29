// Iterative in-place reversal of a singly linked list using three pointers.
// Time: O(n), Space: O(1).
class Node {
  constructor(val) { this.val = val; this.next = null; }
}

function reverse(head) {
  let prev = null;
  while (head) {
    const nxt = head.next;
    head.next = prev;
    prev = head;
    head = nxt;
  }
  return prev;
}

let head = null;
for (let i = 5; i >= 1; i--) { const n = new Node(i); n.next = head; head = n; }
head = reverse(head);
const out = [];
for (let c = head; c; c = c.next) out.push(c.val);
console.log(out.join(" "));
