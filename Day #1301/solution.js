// Day 1301: Fewest jumps from 0 to N where the ith jump moves exactly i (left/right).
// Find smallest k with S=k(k+1)/2 >= |N| and (S-|N|) even (flipping a jump changes sum by 2*val).
function minJumps(N) {
  N = Math.abs(N);
  let k = 0, sum = 0;
  while (sum < N || (sum - N) % 2 !== 0) {
    k++;
    sum += k;
  }
  return k;
}

console.log(minJumps(3)); // 2
console.log(minJumps(2)); // 3
