// Day 1207: Remove kth-from-last node in one pass, constant space.
// Two pointers k apart; when lead hits end, trail is just before target. Time O(n), Space O(1).
class Node {
  constructor(val, next = null) { this.val = val; this.next = next; }
}

function removeKthLast(head, k) {
  const dummy = new Node(0, head);
  let lead = dummy, trail = dummy;
  for (let i = 0; i < k; i++) lead = lead.next;
  while (lead.next) { lead = lead.next; trail = trail.next; }
  trail.next = trail.next.next;
  return dummy.next;
}

let head = new Node(1, new Node(2, new Node(3, new Node(4, new Node(5)))));
head = removeKthLast(head, 2); // remove 4
const out = [];
for (let p = head; p; p = p.next) out.push(p.val);
console.log(out.join(" -> ")); // 1 -> 2 -> 3 -> 5
