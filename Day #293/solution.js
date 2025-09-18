// Pyramid reshape (lowering only): L[i]/R[i] cap ramp slopes, peak v=max min(L,R), cost=sum-v*v.
// Time O(n), Space O(n).
function minCost(h) {
  const n = h.length;
  const L = new Array(n), R = new Array(n);
  L[0] = Math.min(h[0], 1);
  for (let i = 1; i < n; i++) L[i] = Math.min(h[i], L[i-1] + 1);
  R[n-1] = Math.min(h[n-1], 1);
  for (let i = n-2; i >= 0; i--) R[i] = Math.min(h[i], R[i+1] + 1);
  let v = 0, sum = 0;
  for (let i = 0; i < n; i++) { v = Math.max(v, Math.min(L[i], R[i])); sum += h[i]; }
  return sum - v * v;
}

const h = [1, 1, 3, 3, 2, 1];
console.log(minCost(h));
