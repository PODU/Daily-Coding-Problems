// Gray code via reflect-and-prefix formula g(i) = i XOR (i>>1).
// Time: O(n * 2^n) to format. Space: O(2^n).
function grayCode(n) {
  const res = [];
  for (let i = 0; i < (1 << n); i++) {
    const g = i ^ (i >> 1);
    res.push(g.toString(2).padStart(n, "0"));
  }
  return res;
}

console.log("[" + grayCode(2).join(", ") + "]"); // [00, 01, 11, 10]
