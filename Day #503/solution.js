// Day 503: Sort a singly linked list using bottom-up (iterative) merge sort.
// Time O(n log n), Space O(1) auxiliary (no recursion).

class Node {
  constructor(val, next = null) {
    this.val = val;
    this.next = next;
  }
}

function listLength(head) {
  let n = 0;
  for (; head; head = head.next) n++;
  return n;
}

// Split off `size` nodes from head; cut there and return the rest.
function split(head, size) {
  for (let i = 1; head && i < size; i++) head = head.next;
  if (!head) return null;
  const rest = head.next;
  head.next = null;
  return rest;
}

// Merge two sorted lists after `tail`; return new tail.
function mergeLists(a, b, tail) {
  while (a && b) {
    if (a.val <= b.val) { tail.next = a; a = a.next; }
    else { tail.next = b; b = b.next; }
    tail = tail.next;
  }
  tail.next = a ? a : b;
  while (tail.next) tail = tail.next;
  return tail;
}

function sortList(head) {
  if (!head || !head.next) return head;
  const n = listLength(head);
  const dummy = new Node(0);
  dummy.next = head;
  for (let size = 1; size < n; size *= 2) {
    let tail = dummy;
    let cur = dummy.next;
    while (cur) {
      const left = cur;
      const right = split(left, size);
      cur = split(right, size);
      tail = mergeLists(left, right, tail);
    }
  }
  return dummy.next;
}

function printList(head) {
  const parts = [];
  for (; head; head = head.next) parts.push(head.val);
  console.log(parts.join(" -> "));
}

function main() {
  const vals = [4, 1, -3, 99];
  const dummy = new Node(0);
  let tail = dummy;
  for (const v of vals) { tail.next = new Node(v); tail = tail.next; }
  const sorted = sortList(dummy.next);
  printList(sorted); // -3 -> 1 -> 4 -> 99
}

main();
