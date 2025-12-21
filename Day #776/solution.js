// Day 776: Josephus problem. General O(N) recurrence j=(j+k)%i.
// For k=2 an O(log N) closed form is also given. Returns 1-indexed survivor.
function josephus(n, k) {
  let r = 0;
  for (let i = 2; i <= n; i++) r = (r + k) % i;
  return r + 1;
}

function josephusK2(n) {
  let p = 1;
  while (p * 2 <= n) p *= 2;
  return 2 * (n - p) + 1;
}

console.log(josephus(5, 2)); // 3
console.log(josephusK2(5));  // 3
