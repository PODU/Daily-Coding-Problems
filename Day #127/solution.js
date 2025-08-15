// Day 127: Add two numbers stored as reversed-digit linked lists.
// Single pass with carry. O(max(m,n)) time, O(1) extra space.
class Node {
  constructor(val, next = null) {
    this.val = val;
    this.next = next;
  }
}

function addLists(a, b) {
  const dummy = new Node(0);
  let tail = dummy, carry = 0;
  while (a || b || carry) {
    let s = carry;
    if (a) { s += a.val; a = a.next; }
    if (b) { s += b.val; b = b.next; }
    carry = Math.floor(s / 10);
    tail.next = new Node(s % 10);
    tail = tail.next;
  }
  return dummy.next;
}

function build(d) {
  const dummy = new Node(0);
  let t = dummy;
  for (const v of d) { t.next = new Node(v); t = t.next; }
  return dummy.next;
}

function toStr(n) {
  const parts = [];
  while (n) { parts.push(n.val); n = n.next; }
  return parts.join(" -> ");
}

const a = build([9, 9]); // 99
const b = build([5, 2]); // 25
console.log(toStr(addLists(a, b))); // 4 -> 2 -> 1
