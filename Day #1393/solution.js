// Josephus problem via iterative recurrence j(i)=(j(i-1)+k)%i, answer j(N)+1. O(N) time, O(1) space.
'use strict';

function josephus(n, k) {
  let res = 0;
  for (let i = 2; i <= n; i++) res = (res + k) % i;
  return res + 1;
}

function main() {
  console.log(josephus(5, 2));
}

main();
