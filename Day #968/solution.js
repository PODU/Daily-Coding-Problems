// Day 968: Fewest jumps from 0 to N where i-th jump moves +/- i.
// Approach: smallest k with sum 1..k >= |N| and (sum-|N|) even. Time O(sqrt(N)), Space O(1).

function minJumps(n) {
  n = Math.abs(n);
  let k = 0, sum = 0;
  while (sum < n || (sum - n) % 2 !== 0) { k++; sum += k; }
  return k;
}

console.log(minJumps(10)); // 4  (1+2+3+4=10)
