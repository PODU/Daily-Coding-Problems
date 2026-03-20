// Reflected binary Gray code: g(i) = i ^ (i>>1). Time O(2^n), Space O(2^n).
function grayCode(n) {
  const res = [];
  for (let i = 0; i < (1 << n); i++) {
    const g = i ^ (i >> 1);
    res.push(g.toString(2).padStart(n, '0'));
  }
  return res;
}

console.log("[" + grayCode(2).join(", ") + "]");
