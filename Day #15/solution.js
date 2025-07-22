// Reservoir sampling (k=1): keep current pick with prob 1/i as i-th item streams.
// Time: O(n) single pass, Space: O(1).
function pickRandom(stream) {
  let chosen = null;
  let count = 0;
  for (const x of stream) {
    count++;
    if (Math.floor(Math.random() * count) === 0) chosen = x;
  }
  return chosen;
}

const stream = [10, 20, 30, 40, 50];
const freq = {};
for (let t = 0; t < 100000; t++) {
  const s = pickRandom(stream);
  freq[s] = (freq[s] || 0) + 1;
}
console.log("One sample:", pickRandom(stream));
console.log("Approx frequencies over 100000 trials:");
for (const k of Object.keys(freq).sort((a, b) => a - b))
  console.log(`  ${k}: ${(freq[k] / 100000).toFixed(3)}`);
