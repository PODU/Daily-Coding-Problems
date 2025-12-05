// Day 699: Rotate a singly linked list right by k places.
// Approach: close into a ring, then break it (len - k%len) nodes ahead.
// Time O(n), Space O(1).
class Node {
  constructor(val, next = null) { this.val = val; this.next = next; }
}

function rotateRight(head, k) {
  if (!head || !head.next) return head;
  let len = 1, tail = head;
  while (tail.next) { tail = tail.next; len++; }
  k %= len;
  if (k === 0) return head;
  tail.next = head; // ring
  const steps = len - k;
  let newTail = head;
  for (let i = 1; i < steps; i++) newTail = newTail.next;
  const newHead = newTail.next;
  newTail.next = null;
  return newHead;
}

const build = (vals) => { const d = new Node(0); let c = d; for (const x of vals) { c.next = new Node(x); c = c.next; } return d.next; };
const show = (h) => { const o = []; for (; h; h = h.next) o.push(h.val); console.log(o.join(" -> ")); };

show(rotateRight(build([7, 7, 3, 5]), 2));    // 3 -> 5 -> 7 -> 7
show(rotateRight(build([1, 2, 3, 4, 5]), 3)); // 3 -> 4 -> 5 -> 1 -> 2
