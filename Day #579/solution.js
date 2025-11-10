// Min jumps: smallest k with S=k(k+1)/2 >= |N| and (S-|N|) even. Time O(sqrt(N)), Space O(1).
function minJumps(N) {
  const n = Math.abs(N);
  let k = 0, s = 0;
  while (s < n || (s - n) % 2 !== 0) {
    k++;
    s += k;
  }
  return k;
}

console.log("Minimum jumps to reach 10: " + minJumps(10));
