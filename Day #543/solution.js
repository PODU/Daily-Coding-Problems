// Remove kth-from-last node in one pass via two pointers k apart. O(n) time, O(1) space.
class Node {
  constructor(val, next = null) { this.val = val; this.next = next; }
}

function removeKthLast(head, k) {
  const dummy = new Node(0, head);
  let fast = dummy, slow = dummy;
  for (let i = 0; i < k; i++) fast = fast.next;
  while (fast.next) { fast = fast.next; slow = slow.next; }
  slow.next = slow.next.next;
  return dummy.next;
}

let head = new Node(1, new Node(2, new Node(3, new Node(4, new Node(5)))));
head = removeKthLast(head, 2);
const out = [];
for (let p = head; p; p = p.next) out.push(p.val);
console.log(out.join(" "));
