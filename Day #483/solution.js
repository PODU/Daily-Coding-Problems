// Day 483: Josephus problem.
// Iterative recurrence f(i)=(f(i-1)+k)%i in O(n) time, O(1) space.
// Special O(log n) closed form when k=2.
function josephus(n, k) {
  let result = 0; // 0-indexed survivor among 1 person
  for (let i = 2; i <= n; i++) result = (result + k) % i;
  return result + 1; // 1-indexed
}

// O(log n) special case for k == 2.
function josephusK2(n) {
  let p = 1;
  while (p * 2 <= n) p *= 2;
  return 2 * (n - p) + 1;
}

const n = 5, k = 2;
console.log(josephus(n, k)); // 3
