// Partition list: build "less than k" and ">= k" sublists, then join. O(n) time, O(1) extra.
class Node {
  constructor(val, next = null) { this.val = val; this.next = next; }
}

function partition(head, k) {
  const lessDummy = new Node(0), geDummy = new Node(0);
  let less = lessDummy, ge = geDummy;
  for (let cur = head; cur; cur = cur.next) {
    if (cur.val < k) { less.next = cur; less = cur; }
    else { ge.next = cur; ge = cur; }
  }
  ge.next = null;
  less.next = geDummy.next;
  return lessDummy.next;
}

function build(vals) {
  let head = null;
  for (let i = vals.length - 1; i >= 0; i--) head = new Node(vals[i], head);
  return head;
}

function show(head) {
  const parts = [];
  for (let c = head; c; c = c.next) parts.push(c.val);
  return parts.join(" -> ");
}

console.log(show(partition(build([5, 1, 8, 0, 3]), 3)));
