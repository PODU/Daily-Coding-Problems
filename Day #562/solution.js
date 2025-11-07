// Product of array except self without division.
// Prefix * suffix products in two passes. Time O(N), Space O(1) extra (besides output).
function productExceptSelf(a) {
  const n = a.length;
  const res = new Array(n).fill(1);
  let prefix = 1;
  for (let i = 0; i < n; i++) { res[i] = prefix; prefix *= a[i]; }
  let suffix = 1;
  for (let i = n - 1; i >= 0; i--) { res[i] *= suffix; suffix *= a[i]; }
  return res;
}

const fmt = (v) => "[" + v.join(", ") + "]";

console.log(fmt(productExceptSelf([1, 2, 3, 4, 5])));
console.log(fmt(productExceptSelf([3, 2, 1])));
