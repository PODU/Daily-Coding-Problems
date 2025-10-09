// Remove k-th node from end in one pass via two pointers + dummy head. O(n) time, O(1) space.
class Node {
  constructor(val, next = null) { this.val = val; this.next = next; }
}

function removeKthFromEnd(head, k) {
  const dummy = new Node(0, head);
  let fast = dummy, slow = dummy;
  for (let i = 0; i < k; i++) fast = fast.next; // advance fast k ahead
  while (fast.next) { fast = fast.next; slow = slow.next; }
  slow.next = slow.next.next;
  return dummy.next;
}

function printList(head) {
  const parts = [];
  for (let c = head; c; c = c.next) parts.push(c.val);
  console.log(parts.join(" -> "));
}

const head = new Node(1, new Node(2, new Node(3, new Node(4, new Node(5)))));
printList(removeKthFromEnd(head, 2));
