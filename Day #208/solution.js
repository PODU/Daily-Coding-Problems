// Day 208: Partition a linked list around pivot k (stable).
// Build two lists (< k and >= k) in original order, then splice. Time: O(n), Space: O(1).
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
  const dummy = new Node(0);
  let t = dummy;
  for (const x of vals) { t.next = new Node(x); t = t.next; }
  return dummy.next;
}

function toStr(h) {
  const parts = [];
  for (; h; h = h.next) parts.push(h.val);
  return parts.join(" -> ");
}

console.log(toStr(partition(build([5, 1, 8, 0, 3]), 3))); // 1 -> 0 -> 5 -> 8 -> 3
