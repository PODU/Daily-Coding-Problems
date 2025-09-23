// Min jumps on number line: smallest k with S=k(k+1)/2 >= |N| and (S-|N|) even.
// Time: O(sqrt N), Space: O(1).
function minJumps(N) {
  const target = Math.abs(N);
  let k = 0, S = 0;
  while (S < target || (S - target) % 2 !== 0) {
    k++;
    S += k;
  }
  return k;
}

const N = 10;
console.log(`Minimum jumps to reach ${N}: ${minJumps(N)}`);
