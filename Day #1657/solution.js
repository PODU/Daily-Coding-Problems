// Rotate singly linked list right by k. Make ring, break at n-(k%n). O(n) time, O(1) space.
class Node { constructor(v){ this.v = v; this.next = null; } }

function rotateRight(head, k) {
  if (!head || !head.next) return head;
  let n = 1, tail = head;
  while (tail.next) { tail = tail.next; n++; }
  tail.next = head;                 // ring
  const steps = n - (k % n);
  let newTail = head;
  for (let i = 1; i < steps; i++) newTail = newTail.next;
  const newHead = newTail.next;
  newTail.next = null;
  return newHead;
}

function build(xs) {
  let h = null, t = null;
  for (const x of xs) {
    const nd = new Node(x);
    if (!h) { h = t = nd; } else { t.next = nd; t = nd; }
  }
  return h;
}
function show(h) {
  const parts = [];
  for (let p = h; p; p = p.next) parts.push(p.v);
  console.log(parts.join(" -> "));
}

show(rotateRight(build([7, 7, 3, 5]), 2));
show(rotateRight(build([1, 2, 3, 4, 5]), 3));
