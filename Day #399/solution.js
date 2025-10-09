// Partition into 3 contiguous equal-sum parts: greedy prefix cut at target, absorbing trailing zeros. O(n) time, O(n) space.
function partition3(L) {
  const total = L.reduce((a, b) => a + b, 0);
  if (total % 3 !== 0) return null;
  const target = total / 3;
  const res = [];
  let cur = [];
  let running = 0;
  for (let i = 0; i < L.length; i++) {
    cur.push(L[i]);
    running += L[i];
    // close part when sum hits target and next element is non-zero (zeros stay attached)
    if (res.length < 2 && running === target && (i + 1 === L.length || L[i + 1] !== 0)) {
      res.push(cur);
      cur = [];
      running = 0;
    }
  }
  res.push(cur);
  if (res.length !== 3 || res.some(p => p.reduce((a, b) => a + b, 0) !== target)) return null;
  return res;
}

function format(parts) {
  if (parts === null) return "None";
  return "[" + parts.map(p => "[" + p.join(", ") + "]").join(", ") + "]";
}

const L = [3, 5, 8, 0, 8];
console.log(format(partition3(L)));
