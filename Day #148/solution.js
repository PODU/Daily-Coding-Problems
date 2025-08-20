// Gray code generation via reflect formula i ^ (i>>1). Time O(2^n), Space O(2^n).

function grayCode(n) {
  const res = [];
  for (let i = 0; i < (1 << n); i++) {
    const g = i ^ (i >> 1);
    res.push(g.toString(2).padStart(n, '0'));
  }
  return res;
}

const n = 2;
console.log("[" + grayCode(n).join(", ") + "]");
