// Tower of Hanoi: classic recursion. Move n disks from rod `from` to `to` using `via`.
// Prints 2^n - 1 moves. O(2^n) time, O(n) recursion depth.
function hanoi(n, from, to, via) {
  if (n === 0) return;
  hanoi(n - 1, from, via, to);
  console.log(`Move ${from} to ${to}`);
  hanoi(n - 1, via, to, from);
}

hanoi(3, 1, 3, 2);
