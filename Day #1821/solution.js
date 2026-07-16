// Square root via Newton's method. ~O(log(1/eps)) iterations, O(1) space.
function mysqrt(n) {
  if (n < 0) throw new Error("negative");
  if (n === 0) return 0;
  let x = n;
  for (let i = 0; i < 200; i++) {
    const nx = 0.5 * (x + n / x);
    if (Math.abs(nx - x) < 1e-15) { x = nx; break; }
    x = nx;
  }
  return x;
}

const n = 9;
const r = mysqrt(n);
console.log(Math.abs(r - Math.round(r)) < 1e-9 ? Math.round(r) : r);
