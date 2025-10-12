// Day 417: Remove all consecutive nodes summing to zero via prefix-sum + hashmap.
// Time O(n), Space O(n).
class ListNode {
  constructor(val, next = null) {
    this.val = val;
    this.next = next;
  }
}

function removeZeroSum(head) {
  const dummy = new ListNode(0, head);
  const seen = new Map();
  let prefix = 0;
  for (let node = dummy; node; node = node.next) {
    prefix += node.val;
    seen.set(prefix, node); // keep latest node for this prefix sum
  }
  prefix = 0;
  for (let node = dummy; node; node = node.next) {
    prefix += node.val;
    node.next = seen.get(prefix).next; // skip zero-sum run
  }
  return dummy.next;
}

function build(vals) {
  const dummy = new ListNode(0);
  let tail = dummy;
  for (const v of vals) {
    tail.next = new ListNode(v);
    tail = tail.next;
  }
  return dummy.next;
}

let head = removeZeroSum(build([3, 4, -7, 5, -6, 6]));
const parts = [];
for (let n = head; n; n = n.next) parts.push(n.val);
console.log(parts.join(" -> "));
