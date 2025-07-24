// Remove kth-from-last node in one pass with two pointers. Time O(n), Space O(1).
class Node {
  constructor(val) {
    this.val = val;
    this.next = null;
  }
}

function removeKthFromLast(head, k) {
  const dummy = new Node(0);
  dummy.next = head;
  let lead = dummy, trail = dummy;
  for (let i = 0; i < k; i++) lead = lead.next;
  while (lead.next) {
    lead = lead.next;
    trail = trail.next;
  }
  trail.next = trail.next.next;
  return dummy.next;
}

const head = new Node(1);
head.next = new Node(2);
head.next.next = new Node(3);
head.next.next.next = new Node(4);
head.next.next.next.next = new Node(5);
let h = removeKthFromLast(head, 2);
const parts = [];
for (let c = h; c; c = c.next) parts.push(c.val);
console.log(parts.join(" -> "));
