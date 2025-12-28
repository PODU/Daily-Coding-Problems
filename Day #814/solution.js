// Add two numbers stored as reversed-digit linked lists via elementary addition
// with carry, walking both lists. Time O(max(m,n)), Space O(max(m,n)).

class ListNode {
  constructor(val) {
    this.val = val;
    this.next = null;
  }
}

function build(digits) {
  const dummy = new ListNode(0);
  let cur = dummy;
  for (const x of digits) {
    cur.next = new ListNode(x);
    cur = cur.next;
  }
  return dummy.next;
}

function addLists(a, b) {
  const dummy = new ListNode(0);
  let cur = dummy, carry = 0;
  while (a || b || carry) {
    const s = carry + (a ? a.val : 0) + (b ? b.val : 0);
    carry = Math.floor(s / 10);
    cur.next = new ListNode(s % 10);
    cur = cur.next;
    if (a) a = a.next;
    if (b) b = b.next;
  }
  return dummy.next;
}

function toStr(n) {
  const parts = [];
  while (n) {
    parts.push(n.val);
    n = n.next;
  }
  return parts.join(" -> ");
}

function main() {
  const a = build([9, 9]);
  const b = build([5, 2]);
  console.log(toStr(addLists(a, b)));
}

main();
