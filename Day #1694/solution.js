// Zigzag list: single pass, even index wants lst[i]<=lst[i+1], odd wants lst[i]>=lst[i+1]; swap if violated.
// O(n) time, O(1) extra space.

class ListNode {
  constructor(val) {
    this.val = val;
    this.next = null;
  }
}

function zigzag(head) {
  let cur = head;
  let i = 0;
  while (cur && cur.next) {
    if (i % 2 === 0) {
      if (cur.val > cur.next.val) [cur.val, cur.next.val] = [cur.next.val, cur.val];
    } else {
      if (cur.val < cur.next.val) [cur.val, cur.next.val] = [cur.next.val, cur.val];
    }
    cur = cur.next;
    i++;
  }
}

let head = new ListNode(1);
let cur = head;
for (const v of [2, 3, 4, 5]) {
  cur.next = new ListNode(v);
  cur = cur.next;
}

zigzag(head);

const parts = [];
for (cur = head; cur; cur = cur.next) parts.push(cur.val);
console.log(parts.join(" -> "));
