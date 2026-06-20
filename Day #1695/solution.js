// Two-pointer intersection of singly linked lists: redirect each pointer to the other head at end.
// Time O(M+N), Space O(1).

class Node {
  constructor(val) {
    this.val = val;
    this.next = null;
  }
}

function getIntersection(headA, headB) {
  if (!headA || !headB) return null;
  let pA = headA;
  let pB = headB;
  while (pA !== pB) {
    pA = pA === null ? headB : pA.next;
    pB = pB === null ? headA : pB.next;
  }
  return pA;
}

function main() {
  const n8 = new Node(8);
  n8.next = new Node(10);
  const a = new Node(3);
  a.next = new Node(7);
  a.next.next = n8;
  const b = new Node(99);
  b.next = new Node(1);
  b.next.next = n8;

  const res = getIntersection(a, b);
  console.log(res ? res.val : "null");
}

main();
