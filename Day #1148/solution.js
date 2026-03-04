// Day 1148: Rotate linked list right by k.
// Find length, close into ring, cut at (len - k%len). O(n) time, O(1) space.
class Node {
  constructor(val) { this.val = val; this.next = null; }
}

function rotate(head, k) {
  if (!head || !head.next) return head;
  let len = 1, tail = head;
  while (tail.next) { tail = tail.next; len++; }
  k %= len;
  if (k === 0) return head;
  tail.next = head;
  const steps = len - k;
  let newTail = head;
  for (let i = 1; i < steps; i++) newTail = newTail.next;
  const newHead = newTail.next;
  newTail.next = null;
  return newHead;
}

function build(vals) {
  const dummy = new Node(0);
  let t = dummy;
  for (const x of vals) { t.next = new Node(x); t = t.next; }
  return dummy.next;
}

function toStr(h) {
  const out = [];
  for (; h; h = h.next) out.push(h.val);
  return out.join(" -> ");
}

console.log(toStr(rotate(build([7, 7, 3, 5]), 2)));    // 3 -> 5 -> 7 -> 7
console.log(toStr(rotate(build([1, 2, 3, 4, 5]), 3))); // 3 -> 4 -> 5 -> 1 -> 2
