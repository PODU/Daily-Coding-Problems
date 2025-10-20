// Reverse a singly linked list in-place by re-pointing each next pointer.
// Time: O(n), Space: O(1).

class Node {
  constructor(val) {
    this.val = val;
    this.next = null;
  }
}

function reverse(head) {
  let prev = null;
  while (head) {
    const nxt = head.next;
    head.next = prev;
    prev = head;
    head = nxt;
  }
  return prev;
}

function toStr(head) {
  const parts = [];
  while (head) {
    parts.push(head.val);
    head = head.next;
  }
  return parts.join(" ");
}

function main() {
  let head = new Node(1);
  head.next = new Node(2);
  head.next.next = new Node(3);
  head.next.next.next = new Node(4);
  head.next.next.next.next = new Node(5);
  console.log(toStr(head));
  head = reverse(head);
  console.log(toStr(head));
}

main();
