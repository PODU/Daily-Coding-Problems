// Day 1850: rand7() from rand5() via rejection sampling on the 1..25 grid.
// Expected O(1) amortized calls (acceptance 21/25); uniform over 1..7.

function rand5() {
  return Math.floor(Math.random() * 5) + 1;
}

function rand7() {
  while (true) {
    const v = 5 * (rand5() - 1) + rand5(); // 1..25
    if (v <= 21) return ((v - 1) % 7) + 1;
  }
}

function main() {
  const counts = new Array(8).fill(0);
  for (let i = 0; i < 70000; i++) counts[rand7()]++;
  const sample = [];
  for (let i = 0; i < 10; i++) sample.push(rand7());
  console.log("Sample of 10: " + sample.join(" "));
  console.log("Histogram over 70000 draws (each ~10000):");
  for (let i = 1; i <= 7; i++) console.log(`  ${i}: ${counts[i]}`);
}

main();
