// Josephus problem: iterative recurrence J(n)=(J(n-1)+k)%n, plus k=2 closed form.
// Time O(N) (O(log N) for k=2 closed form), Space O(1).
function josephus(n, k) {
  let r = 0;
  for (let i = 2; i <= n; i++) r = (r + k) % i;
  return r + 1;
}
function josephus2(n) { // k==2 closed form
  let p = 1;
  while (p * 2 <= n) p *= 2;
  return 2 * (n - p) + 1;
}
const n = 5, k = 2;
let ans = josephus(n, k);
if (k === 2) ans = josephus2(n);
console.log(ans);
