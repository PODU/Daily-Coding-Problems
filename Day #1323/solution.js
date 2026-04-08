// Day 1323: Min lowering cost to form a pyramid (rise by 1 to a peak, fall by 1, unit ends).
// left[i]=min(a[i],left[i-1]+1), right[i] symmetric; best peak h=max(min(left,right)); pyramid sums to h^2.
// Cost = sum(a) - h^2. O(n) time, O(n) space.

function pyramid(a) {
  const n = a.length;
  const left = new Array(n), right = new Array(n);
  for (let i = 0; i < n; i++) left[i] = Math.min(a[i], (i ? left[i - 1] : 0) + 1);
  for (let i = n - 1; i >= 0; i--) right[i] = Math.min(a[i], (i < n - 1 ? right[i + 1] : 0) + 1);

  let h = 0, peak = 0;
  for (let i = 0; i < n; i++) { const hi = Math.min(left[i], right[i]); if (hi > h) { h = hi; peak = i; } }

  const target = new Array(n).fill(0);
  for (let i = 0; i < n; i++) { const d = Math.abs(i - peak); if (d < h) target[i] = h - d; }
  const cost = a.reduce((s, v) => s + v, 0) - h * h;
  return [cost, target];
}

const [cost, target] = pyramid([1, 1, 3, 3, 2, 1]);
console.log(cost);                      // 2
console.log("[" + target.join(", ") + "]"); // [0, 1, 2, 3, 2, 1]
