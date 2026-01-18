// Reverse a singly linked list in-place by re-pointing each node's next pointer.
// Time: O(n), Space: O(1).
class Node {
  constructor(val, next = null) { this.val = val; this.next = next; }
}

function reverse(head) {
  let prev = null;
  while (head) { const nx = head.next; head.next = prev; prev = head; head = nx; }
  return prev;
}

function toStr(head) {
  const parts = [];
  while (head) { parts.push(head.val); head = head.next; }
  return parts.join(" -> ");
}

let head = new Node(1, new Node(2, new Node(3, new Node(4, new Node(5)))));
head = reverse(head);
console.log(toStr(head)); // 5 -> 4 -> 3 -> 2 -> 1
