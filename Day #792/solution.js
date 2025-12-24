// Pyramid: left[i]=min(h,left[i-1]+1), right[i]=min(h,right[i+1]+1), cap=min(left,right).
// Peak P=max(cap); target descends from P both sides. cost=sum(h)-sum(target). O(n) time/space.
function minPyramidCost(h) {
  const n = h.length;
  const left = new Array(n), right = new Array(n);
  left[0] = Math.min(h[0], 1);
  for (let i = 1; i < n; i++) left[i] = Math.min(h[i], left[i-1] + 1);
  right[n-1] = Math.min(h[n-1], 1);
  for (let i = n - 2; i >= 0; i--) right[i] = Math.min(h[i], right[i+1] + 1);
  const cap = [];
  let P = 0, k = 0;
  for (let i = 0; i < n; i++) {
    cap[i] = Math.min(left[i], right[i]);
    if (cap[i] > P) { P = cap[i]; k = i; }
  }
  const target = new Array(n).fill(0);
  target[k] = P;
  for (let j = 1; k - j >= 0; j++) target[k-j] = Math.max(0, P - j);
  for (let j = 1; k + j < n; j++) target[k+j] = Math.max(0, P - j);
  let cost = 0;
  for (let i = 0; i < n; i++) cost += h[i] - target[i];
  return [cost, target];
}

const [cost, target] = minPyramidCost([1, 1, 3, 3, 2, 1]);
console.log(`${cost} (resulting in [${target.join(", ")}])`);
