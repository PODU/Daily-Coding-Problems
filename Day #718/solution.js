// Day 718: Gray code for n bits via formula g(i) = i ^ (i >> 1). Produces 2^n
// values where consecutive (and wrap-around) differ by one bit. Time O(2^n).
function grayCode(n) {
  const res = [];
  for (let i = 0; i < (1 << n); i++) {
    const g = i ^ (i >> 1);
    res.push(g.toString(2).padStart(n, '0'));
  }
  return res;
}

const codes = grayCode(2);
console.log("[" + codes.join(", ") + "]");
