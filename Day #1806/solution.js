// Tower of Hanoi: recursive divide-and-conquer.
// Time: O(2^n) moves (optimal/minimal). Space: O(n) recursion depth.
function hanoi(n, from, to, via) {
  if (n === 0) return;
  hanoi(n - 1, from, via, to);
  console.log(`Move ${from} to ${to}`);
  hanoi(n - 1, via, to, from);
}

hanoi(3, 1, 3, 2);
