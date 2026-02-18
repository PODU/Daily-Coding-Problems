// Uniformly shuffle a linked list via Fisher-Yates on node values. Time O(n), Space O(n).
// Space-over-time alternative: walk to a random remaining node and swap in place -> O(1) extra, O(n^2) time.
class Node { constructor(v) { this.val = v; this.next = null; } }

function build(arr) {
  let head = null, tail = null;
  for (const x of arr) { const n = new Node(x); if (!head) head = tail = n; else { tail.next = n; tail = n; } }
  return head;
}
function toArr(h) { const a = []; for (let c = h; c; c = c.next) a.push(c.val); return a; }

// Deterministic LCG so the demo is reproducible.
let seed = 42;
function rand(m) { seed = (seed * 1103515245 + 12345) & 0x7fffffff; return seed % m; }

function shuffleList(head) {
  const nodes = [];
  for (let c = head; c; c = c.next) nodes.push(c);
  for (let i = nodes.length - 1; i > 0; i--) {
    const j = rand(i + 1);
    [nodes[i].val, nodes[j].val] = [nodes[j].val, nodes[i].val];
  }
}

const head = build([1, 2, 3, 4, 5]);
console.log('Original: [' + toArr(head).join(', ') + ']');
shuffleList(head);
console.log('Shuffled: [' + toArr(head).join(', ') + ']');
