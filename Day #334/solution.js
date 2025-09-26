// 24 Game (fixed order): recursive split of contiguous list, combine with +,-,*,/ (floats).
// Time: O(1) for fixed 4 numbers. Space: O(1).

function solve(a) {
  if (a.length === 1) return [a[0]];
  const res = [];
  for (let i = 1; i < a.length; i++) {
    const L = solve(a.slice(0, i));
    const R = solve(a.slice(i));
    for (const l of L)
      for (const r of R) {
        res.push(l + r, l - r, l * r);
        if (Math.abs(r) > 1e-9) res.push(l / r);
      }
  }
  return res;
}

function can24(a) {
  return solve(a.map(Number)).some((v) => Math.abs(v - 24.0) < 1e-6);
}

const nums = [5, 2, 7, 8];
console.log(can24(nums) ? "True" : "False");
