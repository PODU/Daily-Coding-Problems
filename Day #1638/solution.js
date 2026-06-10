// Swap every two adjacent nodes in a singly linked list via iterative pointer swaps.
// Time: O(n), Space: O(1).

class Node {
  constructor(val) {
    this.val = val;
    this.next = null;
  }
}

function swapPairs(head) {
  const dummy = new Node(0);
  dummy.next = head;
  let prev = dummy;
  while (prev.next && prev.next.next) {
    const a = prev.next;
    const b = a.next;
    a.next = b.next;
    b.next = a;
    prev.next = b;
    prev = a;
  }
  return dummy.next;
}

function main() {
  let head = new Node(1);
  head.next = new Node(2);
  head.next.next = new Node(3);
  head.next.next.next = new Node(4);

  head = swapPairs(head);

  const parts = [];
  let cur = head;
  while (cur) {
    parts.push(String(cur.val));
    cur = cur.next;
  }
  console.log(parts.join(" -> "));
}

main();
