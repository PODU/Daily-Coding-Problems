// Deep clone linked list with random pointers using O(1) interleaving (3 passes).
// Time O(n), Space O(1) extra. Node.js.

class Node {
  constructor(val) {
    this.val = val;
    this.next = null;
    this.random = null;
  }
}

function cloneList(head) {
  if (!head) return null;
  // Pass 1: insert cloned node after each original
  for (let cur = head; cur; cur = cur.next.next) {
    const copy = new Node(cur.val);
    copy.next = cur.next;
    cur.next = copy;
  }
  // Pass 2: set clone.random
  for (let cur = head; cur; cur = cur.next.next) {
    cur.next.random = cur.random ? cur.random.next : null;
  }
  // Pass 3: split lists
  const newHead = head.next;
  for (let cur = head; cur; cur = cur.next) {
    const copy = cur.next;
    cur.next = copy.next;
    copy.next = copy.next ? copy.next.next : null;
  }
  return newHead;
}

function main() {
  const n1 = new Node(1), n2 = new Node(2), n3 = new Node(3), n4 = new Node(4);
  n1.next = n2; n2.next = n3; n3.next = n4;
  n1.random = n3;
  n2.random = n1;
  n3.random = n3;
  n4.random = n2;

  const cloned = cloneList(n1);

  const origSet = new Set();
  for (let cur = n1; cur; cur = cur.next) origSet.add(cur);

  let deep = true;
  for (let cur = cloned; cur; cur = cur.next) {
    console.log(`node ${cur.val} random ${cur.random ? cur.random.val : 0}`);
    if (origSet.has(cur)) deep = false;
    if (cur.random && origSet.has(cur.random)) deep = false;
  }
  console.log(`deep copy verified: ${deep}`);
}

main();
