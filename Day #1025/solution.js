// Day 1025: Remove all consecutive linked-list nodes that sum to zero.
// Approach: prefix-sum + map of last node per prefix sum (dummy head). O(N) time, O(N) space.
class Node {
  constructor(val, next = null) {
    this.val = val;
    this.next = next;
  }
}

function removeZeroSum(head) {
  const dummy = new Node(0, head);
  const last = new Map();
  let s = 0;
  for (let cur = dummy; cur; cur = cur.next) {
    s += cur.val;
    last.set(s, cur); // keep last occurrence
  }
  s = 0;
  for (let cur = dummy; cur; cur = cur.next) {
    s += cur.val;
    cur.next = last.get(s).next;
  }
  return dummy.next;
}

function build(vals) {
  let head = null;
  for (let i = vals.length - 1; i >= 0; i--) head = new Node(vals[i], head);
  return head;
}

let head = removeZeroSum(build([3, 4, -7, 5, -6, 6]));
const parts = [];
for (let c = head; c; c = c.next) parts.push(c.val);
console.log(parts.join(" -> "));
