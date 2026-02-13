// Day 1062: Swap every two adjacent nodes in a singly linked list.
// Approach: iterative pointer manipulation. Time O(n), Space O(1).

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

function printList(head) {
  const parts = [];
  while (head) {
    parts.push(head.val);
    head = head.next;
  }
  console.log(parts.join(" -> "));
}

let head = new Node(1);
head.next = new Node(2);
head.next.next = new Node(3);
head.next.next.next = new Node(4);
head = swapPairs(head);
printList(head); // 2 -> 1 -> 4 -> 3
