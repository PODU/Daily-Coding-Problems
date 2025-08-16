// Day 128: Tower of Hanoi - print all moves.
// Classic recursion. O(2^n) moves (optimal), O(n) recursion depth.
function hanoi(n, from, via, to) {
  if (n === 0) return;
  hanoi(n - 1, from, to, via);
  console.log(`Move ${from} to ${to}`);
  hanoi(n - 1, via, from, to);
}

hanoi(3, 1, 2, 3);
