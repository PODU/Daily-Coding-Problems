// Fisher-Yates shuffle of linked-list nodes via array: O(n) time, O(n) space.
// Space-over-time tradeoff: O(1)-extra approach repeatedly picks a random node by traversal in O(n^2) time.
class Node { constructor(val){ this.val = val; this.next = null; } }

// deterministic seeded RNG (mulberry32)
function mulberry32(seed){
  return function(){
    seed |= 0; seed = (seed + 0x6D2B79F5) | 0;
    let t = Math.imul(seed ^ (seed >>> 15), 1 | seed);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

function main(){
  let head = null, tail = null;
  for(let v = 1; v <= 5; v++){ const n = new Node(v); if(!head){ head = tail = n; } else { tail.next = n; tail = n; } }
  const a = [];
  for(let p = head; p; p = p.next) a.push(p);
  const n = a.length;
  const rand = mulberry32(12345);
  for(let i = n - 1; i > 0; i--){ const j = Math.floor(rand() * (i + 1)); [a[i], a[j]] = [a[j], a[i]]; }
  for(let i = 0; i < n; i++) a[i].next = (i + 1 < n ? a[i + 1] : null);
  head = a[0];
  const orig = [1, 2, 3, 4, 5];
  const shuf = [];
  for(let p = head; p; p = p.next) shuf.push(p.val);
  shuf.sort((x, y) => x - y);
  const valid = orig.every((v, i) => v === shuf[i]);
  console.log(orig.join(" ") + " -> " + (valid ? "valid shuffle (same elements)" : "INVALID"));
}
main();
