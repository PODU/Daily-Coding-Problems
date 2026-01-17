// Reflected binary Gray code: value i -> i ^ (i>>1) for i in 0..2^n. O(2^n) time/space.
function grayCode(n) {
  const res = [];
  const total = 1 << n;
  for (let i = 0; i < total; i++) {
    const g = i ^ (i >> 1);
    res.push(g.toString(2).padStart(n, "0"));
  }
  return res;
}

const n = 2;
console.log("[" + grayCode(n).join(", ") + "]");
