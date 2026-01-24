// Day 948: Tower of Hanoi - print all moves to move n disks from rod 1 to rod 3.
// Classic recursion. Time O(2^n) moves, Space O(n) recursion depth.

function hanoi(n, from, to, aux) {
  if (n === 0) return;
  hanoi(n - 1, from, aux, to);
  console.log(`Move ${from} to ${to}`);
  hanoi(n - 1, aux, to, from);
}

hanoi(3, 1, 3, 2);
