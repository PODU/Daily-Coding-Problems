// Uniform linked-list shuffle via Fisher-Yates. Time O(n), Space O(n) for the
// index pass (space-over-time variant: O(1) space, O(n^2) by random node selection).
// A deterministic LCG (BigInt for exact 64-bit math) is used so output is reproducible.
class Node {
  constructor(val) { this.val = val; this.next = null; }
}

let seed = 42n;
function nextRand() {
  seed = (seed * 1103515245n + 12345n) % 2147483648n;
  return seed;
}

function shuffle(head) {
  const nodes = [];
  for (let p = head; p; p = p.next) nodes.push(p);
  for (let i = nodes.length - 1; i >= 1; i--) {
    const j = Number(nextRand() % BigInt(i + 1));
    [nodes[i].val, nodes[j].val] = [nodes[j].val, nodes[i].val];
  }
  return head;
}

let head = null, tail = null;
for (let v = 1; v <= 5; v++) {
  const node = new Node(v);
  if (!head) head = tail = node;
  else { tail.next = node; tail = node; }
}
head = shuffle(head);
const out = [];
for (let p = head; p; p = p.next) out.push(p.val);
console.log(out.join(" -> "));
