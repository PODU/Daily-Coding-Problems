// Presence bitset: mark each present value, then report unmarked ones.
// Time: O(N), Space: O(N) bits.  (N = 1,000,000)
function findMissing(present, N) {
  const seen = new Uint8Array(N + 1);
  for (const x of present) seen[x] = 1;
  const missing = [];
  for (let i = 1; i <= N; i++) if (!seen[i]) missing.push(i);
  return missing;
}

const N = 1000000;
const present = [];
for (let i = 1; i <= N; i++) if (i % 1000 !== 0) present.push(i);

const missing = findMissing(present, N);
console.log("Missing count:", missing.length);
console.log("First 10 missing:", missing.slice(0, 10).join(" "));
console.log("Time complexity: O(N), Space complexity: O(N) bits (N = 1,000,000)");
